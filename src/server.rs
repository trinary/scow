use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
use tokio::time::{self, Duration};

use std::future::Future;
use std::sync::Arc;

use tracing::{debug, error, info};

use crate::command::{Command, Response};
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
            println!("got to server.run?");
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
        Shutdown {
            shutdown: false,
        }
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
        println!("in Handler#run, should have something on the wire");
        while !self.shutdown.is_shutdown() {
            let maybe_frame = tokio::select! {
                res = self.connection.read_command() => res?
            };

            let cmd = match maybe_frame {
                Some(cmd) => cmd,
                None => {
                    debug!("didn't get a command, returning from Handler#run");
                    return Ok(());
                }
            };
            debug!(?cmd);
            let result = match cmd {
                Command::Read(k) => {
                    let read = self.db.get(&k);
                    Response::Value(read.unwrap())
                }
                Command::Write(k, v) => {
                    let _write = self.db.set(k, v);
                    Response::Success
                }
            };
            self.connection.write(result.to_string().as_str()).await?;
        }
        Ok(())
    }
}
