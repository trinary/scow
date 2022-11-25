
use scow::protocol::Connection;

pub struct Client {
    connection: Connection,
}

impl Client {
    pub fn connect(addr: String) -> bool {
        true
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::connect(String::from("127.0.0.1:9999"));
}
