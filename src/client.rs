use crate::command::Frame;
use crate::connection::{Connection, Result};
use tokio::net::{TcpStream, ToSocketAddrs};

use std::io::Cursor;

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(Client { connection })
    }

    pub async fn get(&mut self, key: &str) -> Result<()> {
        println!("client writing GET command");
        self.connection
            .write(format!("r{}\r\n", key).as_str())
            .await?;

        println!("client wrote GET. waiting on response.");
        self.read_response_frame().await
    }

    pub async fn set(&mut self, key: &str, val: &str) -> Result<()> {
        println!("client writing SET command");
        self.connection
            .write(format!("w{} {}\r\n", key, val).as_str())
            .await?;
        println!("client wrote SET. waiting on response.");
        self.read_response_frame().await
    }

    async fn read_response_frame(&mut self) -> Result<()> {
        loop {
            println!("client read_response");
            let mut cursor = Cursor::new(&self.connection.buffer[..]);
            println!("client read_response made a cursor");
            let response = Frame::parse(&mut cursor)?;
            println!("got response data");
            match response {
                Frame::Success => {
                    println!("client read_response match got Success");
                    return Ok(());
                }
                Frame::Error(e) => {
                    println!("client read_response match got error(e): {:?}", e);
                    return Ok(());
                }
                Frame::Value(v) => {
                    println!("client read_response match got value(v): {:?}", v);
                    return Ok(());
                }
                Frame::Read(k) => {
                    println!("client read_response match got read(k): {:?}", k);
                    return Ok(());
                }
                Frame::Write(k, v) => {
                    println!("client read_response match got write(k, v): {:?}, {:?}", k, v);
                    return Ok(());
                }
            }
        }
    }
}
