// put client commands here.
// this is for stuff like reads, writes, info requests (get the leader's addr)

pub enum Command {
    Read(i32),
    Write(i32, String),
}

#[derive(Clone, Debug)]
pub enum Response {
    Ok,
    Value(String),
    Error(String),
}
