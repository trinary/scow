
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

    pub async fn get(&mut self, key: &str) -> crate::Result<()> {
        println!("client writing GET command");
        self.connection.write(format!("r{}\r\n", key).as_str()).await?;

        println!("client wrote. waiting on response.");
        self.read_response().await
    }

    async fn read_response(&mut self) -> crate::Result<()> {
        println!("client read_response");
        let response = self.connection.read_command().await?;
        match response {
            Some(v) => println!("client read_response match got: {:?}", v),
            _ => println!("client read_response got nothing."),
        }
        println!("client read_response match block finished.");
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9999);

    let _res = match Client::connect(&addr).await {
        Ok(mut cl) => {
            println!("success! cl is {:?}", cl.connection);
            let val = cl.get("key").await;
            val
        },
        Err(e) => {
            println!("oh no: {:?}", e);
            Err(e)
        }
    };
    println!("idk, end of the line");
}
