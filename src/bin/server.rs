use std::io;
use tokio::net::{TcpListener, TcpStream};

use scow::connection::Connection;


#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    println!("listening");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("OH SHIT WHATUP");
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    let command = connection.read_command().await;
    println!("Got a command: {:?}", command);

}
