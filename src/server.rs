use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use tokio::time;

use std::future::Future;
use std::net::{SocketAddrV4, Ipv4Addr};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tracing::{debug, error, info};

use crate::command::Frame;
use crate::connection::{Connection, Result};
use crate::consensus::{TermState, ServerId};
use crate::handler::{Db, DbDropGuard};


pub async fn run(tcp_listener: TcpListener, shutdown: impl Future) {

    let mut server = Server {
        tcp_listener,
        db_holder: DbDropGuard::new(),
        limit_connections: Arc::new(Semaphore::new(100)),
        term_state: TermState::new(),
        last_heartbeat: SystemTime::UNIX_EPOCH,
        heartbeat_interval: 5000
    };

    tokio::select! {
     res = server.run() => {
         debug!("got to server.run?");
         if let Err(err) = res {
             error!(cause = %err, "failed to accept");
         }
     },
     _ = shutdown => {
         info!("shutdown");
     },
    }

}

#[derive(Debug)]
struct Server {
    tcp_listener: TcpListener,
    db_holder: DbDropGuard,
    limit_connections: Arc<Semaphore>,
    term_state: TermState,
    heartbeat_interval: u64,
    last_heartbeat: SystemTime,
}

impl Server {
    async fn run(&mut self) -> Result<()> {
        info!("Accepting inbound connections");

        let servers = vec![
            ServerId {
                id: 0,
                address: std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 9990)),
            },
            ServerId {
                id: 1,
                address: std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 9991)),
            },
        ];

        loop {
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();

            let socket = self.accept().await?;

            let mut handler = Handler {
                db: self.db_holder.db(),
                connection: Connection::new(socket),
                shutdown: Shutdown::new(),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!(cause = ?err, "connection error");
                    drop(permit);
                }
                
            });

            tokio::spawn(async move {
                
            });
        }
    }

    async fn accept(&mut self) -> Result<TcpStream> {
        debug!("accept in listener");
        let mut backoff = 1;
        loop {
            match self.tcp_listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(e) => {
                    if backoff > 64 {
                        return Err(e.into());
                    }
                }
            }

            time::sleep(Duration::from_secs(backoff)).await;
            backoff *= 2;
        }
    }

    // async fn heartbeat_loop(&mut self) -> () {
    //     let heartbeat = tokio::spawn(async move  {
    //         if let Err(err) = self.heartbeat().await {
    //             error!(cause = ?err, "heartbeat error");
    //         }
    //     });
    // }

    pub async fn heartbeat(&mut self) -> Result<String> {
        println!("heartbeat loop start");
        let mut interval = tokio::time::interval(Duration::from_millis(self.heartbeat_interval));

        loop {
            interval.tick().await;
            self.heartbeat_action().await;
        }
    }

    async fn heartbeat_action(&mut self) {
        // do the things here
        println!("thump-thump");
        let now = SystemTime::now();

        // the algorithm:

        // IF this server is not the leader
        //   AND this server has not gotten a ping from the leader (within random timeout)

        if self.term_state.server_state == crate::consensus::ServerState::Follower {
            if self.last_heartbeat.elapsed().unwrap() > Duration::from_millis(250) {

            // THEN 
            //   - change state to candidate  
            //   - increase term counter
            //   - vote for yourself
            //   - request votes from known servers

                println!("request vote");
            }
        }
    }

}

pub(crate) struct Shutdown {
    shutdown: bool,
}

impl Shutdown {
    pub(crate) fn new() -> Shutdown {
        Shutdown { shutdown: false }
    }
    pub(crate) fn is_shutdown(&self) -> bool {
        self.shutdown
    }
}

struct Handler {
    db: Db,
    connection: Connection,
    shutdown: Shutdown,
}

impl Handler {
    async fn run(&mut self) -> crate::connection::Result<()> {
        debug!("in Handler#run, should have something on the wire");


        while !self.shutdown.is_shutdown() {
            // TODO should make this kind of stuff part of the frame or connection types
            // maybe add some kind of more specific command and response-handling hook
            // like a handler for each type of command for server side, handler for each type of response for client


            // all commands and responses being in the same frame type is a little weird?
            // should frame be union of a single command OR a single response?
            let maybe_frame = tokio::select! {
                res = self.connection.read_frame() => res?
            };

            let frame = match maybe_frame {
                Some(cmd) => cmd,
                None => {
                    debug!("didn't get a command, returning from Handler#run");
                    return Ok(());
                }
            };
            debug!(?frame);
            let result = match frame {
                Frame::Read(k) => {
                    let read = self.db.get(&k);
                    match read {
                        Some(v) => Frame::Value(v),
                        None => Frame::Error(String::from("Key not found.")),
                    }
                }
                Frame::Write(k, v) => {
                    let _write = self.db.set(k, v);
                    Frame::Success
                }
                Frame::Success => Frame::Success,
                Frame::RequestVote(_) => {
                    todo!()
                },
                Frame::Vote(server) => {
                    todo!()
                },
                Frame::Value(_) => {
                    // servers dont' need to care about this type of frame, but we should handle it eventually
                    todo!()
                }
                Frame::Error(_) => todo!(),
            };
            let response = result.to_string();
            println!("writing {} to the wire.", response);
            self.connection.write(&response).await?;
        }
        Ok(())
    }
}
