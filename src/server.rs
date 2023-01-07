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
                connection: Connection::new(socket)
            };
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

struct Handler {
    pub db: Db,
    pub connection: Connection,
}

impl Handler {
    // TODO: this impl
 }