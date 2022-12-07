
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use scow::client::Client;
use scow::connection::Result;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9999);

    let _res = match Client::connect(&addr).await {
        Ok(mut cl) => {
            println!("connected!");
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
