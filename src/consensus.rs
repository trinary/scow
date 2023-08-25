// put consensus comamnds and data structures here.
// this is for terms, leader elections, etc.

use std::fmt::Display;
use std::net::SocketAddr;

pub struct Entry {
    key: String,
    value: String,
}

// pub enum ServerCommand {
//     AppendEntries(Box<[Entry]>),
//     RequestVote,
// }

#[derive(Debug, PartialEq)]
pub enum ServerState {
    Leader,
    Follower,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ServerId {
    pub id: u32,
    pub address: SocketAddr,
}

impl Display for ServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.id, self.address)
    }
}

#[derive(Debug)]
pub struct TermState {
    pub current_term: u64,
    pub server_state: ServerState,
    pub leader: Option<ServerId>,
}

impl TermState {
    pub fn new() -> TermState {
        TermState {
            current_term: 0,
            server_state: ServerState::Follower,
            leader: None,
        }
    }
}