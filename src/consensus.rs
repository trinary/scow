// put consensus comamnds and data structures here.
// this is for terms, leader elections, etc.

pub struct Entry {
    key: String,
    value: String,
}

pub enum ServerCommand {
    AppendEntries(Box<[Entry]>),
    RequestVote,
}

pub struct ServerState {
    current_term: i128,
    voted_for: ServerId,
}

pub struct ServerId {
    id: i32,
    address: String,
}