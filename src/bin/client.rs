
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use scow::protocol::Connection;
use scow::protocol::Result;

use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    connection: Connection,
}

impl Client {
    pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(Client { connection })
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9999);

    let client = match Client::connect(&addr).await {
        Ok(cl) => println!("success! cl is {:?}", cl.connection),
        Err(e) => println!("oh no: {}", e),
    };
}
