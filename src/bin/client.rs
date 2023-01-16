
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use scow::client::Client;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9999);

    let _res = match Client::connect(&addr).await {
        Ok(mut cl) => {
            println!("connected!");
            let set_result = cl.set("key", "value").await;
            println!("got a result from PUT: {:?}", set_result);
            let get_result = cl.get("key").await;
            println!("got a result from GET: {:?}", get_result);
            
            get_result
        },
        Err(e) => {
            println!("oh no: {:?}", e);
            Err(e)
        }
    };
    println!("idk, end of the line");
}
