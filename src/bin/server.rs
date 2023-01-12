use std::io;
use tokio::net::TcpListener;
use tokio::signal;


#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    
    println!("listening");
    scow::server::run(listener, signal::ctrl_c()).await;
    io::Result::Ok(())
}