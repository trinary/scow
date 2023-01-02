use std::io;
use tokio::{net::{TcpListener, TcpStream}, signal};

use scow::connection::Connection;


#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    
    println!("listening");
    scow::server::run(listener, signal::ctrl_c()).await;
    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //     println!("OH SHIT WHATUP");
    //     process(socket).await;
    // }
    io::Result::Ok(())
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    let command = connection.read_command().await;
    println!("Got a command: {:?}", command);

}
