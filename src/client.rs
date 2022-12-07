use crate::connection::{Connection, Result};
use tokio::net::{TcpStream, ToSocketAddrs};

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
        self.connection.write(format!("r{}\r\n", key).as_str()).await?;

        println!("client wrote. waiting on response.");
        self.read_response().await
    }

    async fn read_response(&mut self) -> Result<()> {
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