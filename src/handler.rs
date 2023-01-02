use std::collections::HashMap;
use crate::connection::Connection;

pub struct Handler {
    pub db: HashMap<String, String>,
    pub connection: Connection,
}