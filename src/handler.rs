use std::collections::HashMap;
use crate::connection::Connection;


struct Handler {
    db: HashMap<String, String>,
    connection: Connection,
}