use std::io;
use std::env;
use tokio::net::{TcpListener, TcpStream};

use scow::protocol::{Connection};

#[tokio::main]
async fn main() -> io::Result<()> {
    let _args: Vec<String> = env::args().collect();
    //let port: i32 = args[1].parse::<i32>().unwrap();

    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    let command = connection.read_command().await;
    println!("Got a command: {:?}", command);

}
