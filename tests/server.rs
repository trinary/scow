use std::net::SocketAddr;
use tokio::net::TcpListener;

use scow::command::Frame;
use scow::{client::Client, server};

#[tokio::test]
async fn set_then_get() {
    let addr = start_server().await;
    let mut client = Client::connect(addr).await.unwrap();

    let set_result = client.write("key", "testval").await.unwrap();
    assert_eq!(set_result, Frame::Success);

    let set_results2 = client.write("key2", "testval2").await.unwrap();
    assert_eq!(set_results2, Frame::Success);

    let get_result = client.read("key").await.unwrap();
    assert_eq!(get_result, Frame::Value("testval".to_string()));
}

#[tokio::test]
async fn unknown_key() {
    let addr = start_server().await;
    let mut client = Client::connect(addr).await.unwrap();

    let set_result = client.read("unknown").await.unwrap();
    assert_eq!(set_result, Frame::Error("Key not found.".to_string()));
}

async fn start_server() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move { server::run(listener, tokio::signal::ctrl_c()).await });
    addr
}
