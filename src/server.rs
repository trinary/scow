use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use tokio::time::{self, Duration};

use std::future::Future;
use std::sync::Arc;

use tracing::{debug, error, info};

use crate::command::Frame;
use crate::connection::{Connection, Result};
use crate::handler::{Db, DbDropGuard};

pub async fn run(tcp_listener: TcpListener, shutdown: impl Future) {
    let mut server = Server {
        tcp_listener,
        db_holder: DbDropGuard::new(),
        limit_connections: Arc::new(Semaphore::new(100)),
    };

    tokio::select! {
        res = server.run() => {
            debug!("got to server.run?");
            if let Err(err) = res {
                error!(cause = %err, "failed to accept");
            }

        }
            _ = shutdown => {
            info!("shutdown");
        }
    }
}

#[derive(Debug)]
struct Server {
    tcp_listener: TcpListener,
    db_holder: DbDropGuard,
    limit_connections: Arc<Semaphore>,
}

impl Server {
    async fn run(&mut self) -> Result<()> {
        info!("Accepting inbound connections");

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

            // though

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
                    Frame::Value(read.unwrap())
                }
                Frame::Write(k, v) => {
                    let _write = self.db.set(k, v);
                    Frame::Success
                }
                Frame::Success => Frame::Success,
                Frame::Value(_) => {
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
