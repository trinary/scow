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

    pub async fn get(&mut self, key: &str) -> Result<Frame> {
        debug!("client writing GET command");
        self.connection
            .write(format!("r{}\r\n", key).as_str())
            .await?;

        debug!("client wrote GET. waiting on response.");
        self.read_response_frame().await
    }

    pub async fn set(&mut self, key: &str, val: &str) -> Result<Frame> {
        debug!("client writing SET command");
        self.connection
            .write(format!("w{} {}\r\n", key, val).as_str())
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
            // match frame {
            //     Frame::Success => {
            //         println!("client read_response match got Success");
            //         return Ok(frame);
            //     }
            //     Frame::Error(e) => {
            //         println!("client read_response match got error(e): {:?}", e);
            //         return Ok(frame);
            //     }
            //     Frame::Value(v) => {
            //         println!("client read_response match got value(v): {:?}", v);
            //         return Ok(frame);
            //     }
            //     Frame::Read(k) => {
            //         println!("client read_response match got read(k): {:?}", k);
            //         return Ok(frame);
            //     }
            //     Frame::Write(k, v) => {
            //         println!(
            //             "client read_response match got write(k, v): {:?}, {:?}",
            //             k, v
            //         );
            //         return Ok(frame);
            //     }
            // }
        }
    }
}
