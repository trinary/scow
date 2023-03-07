use tokio::net::TcpListener;
use std::net::SocketAddr;

use scow::{server, client::Client};
use scow::command::Frame::*;

#[tokio::test]
async fn set_then_get() {
    let addr = start_server().await;
    let mut client = Client::connect(addr).await.unwrap();

    let set_result = client.set("key", "testval").await.unwrap();
    assert_eq!(set_result, Success)
}

async fn start_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move { 
        server::run(listener, tokio::signal::ctrl_c()).await
    });
    addr
}