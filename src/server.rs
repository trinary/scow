use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Semaphore};
use tokio::time::{self, Duration};

use std::future::Future;
use std::sync::Arc;

use tracing::{debug, error, info};

use crate::connection::{Connection, Result};
use crate::handler::{Db, DbDropGuard};

pub async fn run(tcp_listener: TcpListener, shutdown: impl Future) {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
    let mut server = Server {
        tcp_listener,
        db_holder: DbDropGuard::new(),
        limit_connections: Arc::new(Semaphore::new(100)),
        notify_shutdown,
        shutdown_complete_rx,
        shutdown_complete_tx,
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
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_rx: mpsc::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
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
                shutdown: Shutdown::new(self.notify_shutdown.subscribe())
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
    notify: broadcast::Receiver<()>,
}

impl Shutdown {
    pub(crate) fn new(notify: broadcast::Receiver<()>) -> Shutdown {
        Shutdown { shutdown: false, notify: notify }
    }
    pub(crate) fn is_shutdown(&self) -> bool {
        false
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
                    return Ok(())
                }
            };
            debug!(?cmd);
        }
        Ok(())
    }
 }