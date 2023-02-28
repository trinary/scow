use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::io::AsyncWriteExt;
use tracing::debug;

use tokio::io::AsyncReadExt;
use tokio::io::BufWriter;
use tokio::net::TcpStream;

use crate::command::{CmdError, Frame};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Connection {
    pub stream: BufWriter<TcpStream>,
    pub buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            debug!("read loop, state: {:?}", self.buffer);
            if let Some(cmd) = self.parse_frame()? {
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

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        debug!("parse_frame");
        let mut buf = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let command = Frame::parse(&mut buf)?;
                self.buffer.advance(len);
                debug!("got a command from check: {:?}", command);
                Ok(Some(command))
            }
            Err(CmdError::Incomplete) => {
                debug!("got incomplete from check");
                Ok(None)
            }
            Err(other) => Err(format!(
                "got an 'other' error in connection#parse_command: {:?}",
                other
            )
            .into()),
        }
    }

    pub async fn write(&mut self, src: &str) -> std::io::Result<()> {
        self.stream.write_all(src.as_bytes()).await?;
        self.stream.flush().await
    }
}
