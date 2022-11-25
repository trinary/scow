use bytes::{Buf, BytesMut};
use tokio::io::BufWriter;
use tokio::net::TcpStream;

mod client;
use crate::protocol::client::Command;
use crate::protocol::client::Response;

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

    pub async fn read_command(&mut self) -> Command {
        Command::Read(1)
    }

    pub async fn execute_command(command: Command) -> Response {
        Response::Ok
    }
}
