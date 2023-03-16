use crate::command::Frame;
use crate::connection::{Connection, Result};
use tokio::net::{TcpStream, ToSocketAddrs};
use tracing::debug;

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(Client { connection })
    }

    pub async fn read(&mut self, key: &str) -> Result<Frame> {
        debug!("client writing GET command");
        let frame = Frame::Read(key.to_string());
        println!("sending {} over the wire!", frame);
        self.connection
            .write(format!("{}", frame).as_str())
            .await?;

        debug!("client wrote GET. waiting on response.");
        self.read_response_frame().await
    }

    pub async fn write(&mut self, key: &str, val: &str) -> Result<Frame> {
        debug!("client writing SET command");
        let frame = Frame::Write(key.to_string(), val.to_string());
        println!("sending {} over the wire!", frame);
        self.connection
            .write(format!("{}", frame).as_str())
            .await?;

        debug!("client wrote SET. waiting on response.");
        self.read_response_frame().await
    }

    async fn read_response_frame(&mut self) -> Result<Frame> {
        loop {
            debug!("client read_response");

            let maybe_frame = tokio::select! {
                res = self.connection.read_frame() => res?
            };

            let frame = match maybe_frame {
                Some(frame) => frame,
                None => {
                    debug!("didn't get a response frame?");
                    todo!();
                }
            };

            return Ok(frame);
        }
    }
}
