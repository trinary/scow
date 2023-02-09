// put client commands here.
// this is for stuff like reads, writes, info requests (get the leader's addr)

use std::fmt;
use std::io::Cursor;
use std::string::FromUtf8Error;

use crate::connection::Error;
use bytes::Buf;

#[derive(Clone, Debug)]
pub enum Command {
    Read(String),
    Write(String, String),
}

#[derive(Debug)]
pub enum CmdError {
    Incomplete,
    Other(Error),
}

#[derive(Clone, Debug)]
pub enum Response {
    Success,
    Value(String),
    Error(String),
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Success => write!(f, "OK"),
            Response::Value(s) => write!(f, "GET {}", s),
            Response::Error(e) => write!(f, "ERR {}", e),
        }
    }
}

impl Response {
    pub(crate) fn parse_response(src: &mut Cursor<&[u8]>) -> Result<Response, &'static str> {
        println!("Response#parse_response, maybe about to crash lol");
        if src.has_remaining() {   
            match src.get_u8() {
                b'O' => Ok(Response::Success),
                b'G' => Ok(Response::Value(String::from("lol"))),
                b'E' => Ok(Response::Error(String::from("error parsing response"))),
                _ => Err("got unexpected data in Response#parse_response"),
            }
        } else {
            println!("client cursor has no data");
            Err("client cursor has no data.")
        }
    }
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

impl From<&str> for CmdError {
    fn from(src: &str) -> CmdError {
        src.to_string().into()
    }
}

impl From<String> for CmdError {
    fn from(src: String) -> CmdError {
        CmdError::Other(src.into())
    }
}

impl From<FromUtf8Error> for CmdError {
    fn from(_src: FromUtf8Error) -> CmdError {
        "protocol error, invalid format (fromUtf8)".into()
    }
}

impl Command {
    pub(crate) fn check(src: &mut Cursor<&[u8]>) -> Result<Option<Command>, CmdError> {
        println!("check");
        match get_u8(src)? {
            b'r' => {
                println!("u8 read r command");
                let line = get_line(src)?;
                let linestr = match String::from_utf8(line.to_vec()) {
                    Ok(v) => v,
                    Err(e) => panic!("invalid utf-8 syntax in read cmd: {:?}", e),
                };
                Ok(Some(Command::Read(linestr)))
            }
            b'w' => {
                println!("u8 read w command");
                get_line(src)?;
                Ok(Some(Command::Write(
                    String::from("bar"),
                    String::from("baz"),
                )))
            }
            other => {
                println!("check - other = {}", other);
                Err(format!("protocol error, unexpected byte `{}`", other).into())
            }
        }
    }

    pub(crate) fn parse(src: &mut Cursor<&[u8]>) -> Result<Command, CmdError> {
        println!("parse");
        match get_u8(src)? {
            b'r' => {
                // TODO: DRY this out?
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                println!("got read line off the wire: {}", string);
                Ok(Command::Read(string))
            }
            b'w' => {
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                println!("got write line off the wire: {}", string);
                let (key, val) = string.split_once(' ').unwrap();
                Ok(Command::Write(String::from(key), String::from(val)))
            }
            _ => unimplemented!(),
        }
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, CmdError> {
    println!("get_u8");
    if !src.has_remaining() {
        println!("get_u8 has no remaining data, returning Incomplete.");
        return Err(CmdError::Incomplete);
    }
    println!("get_u8 has_remaining()");
    Ok(src.get_u8())
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], CmdError> {
    println!("get line");
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;

    println!("start: {} end: {}", start, end);
    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(CmdError::Incomplete)
}
