use scow::client::Client;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9999);

    let _res = match Client::connect(&addr).await {
        Ok(mut cl) => {
            println!("connected!");
            let set_result = cl.set("key", "wheeee").await;
            println!("got a result from PUT: {:?}", set_result);
            let get_result = cl.get("key").await;
            println!("got a result from GET: {:?}", get_result);
            let get_missing_result = cl.get("missing").await;
            println!(
                "what happens when we get a missing val: {:?}",
                get_missing_result
            );

            get_result
        }
        Err(e) => {
            println!("oh no: {:?}", e);
            Err(e)
        }
    };
    println!("idk, end of the line")
}
