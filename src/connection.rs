use bytes::{Buf, BytesMut};
use tokio::io::AsyncWriteExt;
use std::io::Cursor;

use tokio::io::AsyncReadExt;
use tokio::io::BufWriter;
use tokio::net::TcpStream;

use crate::command::{Command, CmdError, Response};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_command(&mut self) -> Result<Option<Command>> {
        loop {
            println!("read loop, state: {:?}", self.buffer);
            if let Some(cmd) = self.parse_command()? {
                return Ok(Some(cmd));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    fn parse_command(&mut self) -> Result<Option<Command>> {
        println!("parse_command");
        let mut buf = Cursor::new(&self.buffer[..]);
        match Command::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let command = Command::parse(&mut buf)?;
                self.buffer.advance(len);
                Ok(Some(command))
            }
            Err(CmdError::Incomplete) => {
                println!("server got incomplete from check");
                Ok(None)
            }
            Err (_other) => Err("uhhhh what".into())
        }
    }

    pub async fn execute_command(command: Command) -> Result<Response> {
        println!("server executing request: {:?}", command);
        Ok(Response::Success)
    }

    pub async fn write(&mut self, src: &str) -> std::io::Result<()> {
        self.stream.write_all(src.as_bytes()).await?;
        self.stream.flush().await
    }
}
