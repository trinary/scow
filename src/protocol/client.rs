// put client commands here.
// this is for stuff like reads, writes, info requests (get the leader's addr)

use std::io::Cursor;
use std::fmt;

use bytes::Buf;

#[derive(Clone, Debug)]
pub enum Command {
    Read(i32),
    Write(i32, String),
}

#[derive(Debug)]
pub enum CmdError {
    Incomplete,
    Other(crate::protocol::Error),
}

impl std::error::Error for CmdError {}
impl std::fmt::Display for CmdError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CmdError::Incomplete => "ended early".fmt(fmt),
            CmdError::Other(err) => err.fmt(fmt),
        }
    }
}

impl From<String> for CmdError {
    fn from(src: String) -> CmdError {
        CmdError::Other(src.into())
    }
}

impl Command {
    pub(crate) fn check(src: &mut Cursor<&[u8]>) -> Result<(), CmdError> {
        match get_u8(src)? {
            b'r' => {
                get_line(src)?;
                Ok(())
            }
            other => 
                Err(format!("protocol error, unexpected byte `{}`", other).into()),
    
        }
    }

    pub(crate) fn parse(src: &mut Cursor<&[u8]>) -> Result<Command, CmdError> {
        Ok(Command::Read(1))
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, CmdError> {
    if !src.has_remaining() {
        return Err(CmdError::Incomplete);
    }
    Ok(src.get_u8())
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], CmdError> {
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);
            return Ok(&src.get_ref()[start..i]);
        }
    }
    
    Err(CmdError::Incomplete)
}

#[derive(Clone, Debug)]
pub enum Response {
    Success,
    Value(String),
    Error(String),
}
