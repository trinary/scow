// put client commands here.
// this is for stuff like reads, writes, info requests (get the leader's addr)

use std::fmt;
use std::io::Cursor;
use std::string::FromUtf8Error;

use crate::connection::Error;
use bytes::Buf;
use tracing::debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Frame {
    Read(String),
    Write(String, String),
    Success,
    Value(String),
    Error(String),
}

#[derive(Debug)]
pub enum CmdError {
    Incomplete,
    Other(Error),
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Frame::Read(s) => write!(f, "READ {}\r\n", s),
            Frame::Write(s, v) => write!(f, "WRITE {} {}\r\n", s, v),
            Frame::Success => write!(f, "OK\r\n"),
            Frame::Value(s) => write!(f, "GET {}\r\n", s),
            Frame::Error(e) => write!(f, "ERR {}\r\n", e),
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

impl Frame {
    pub(crate) fn check(src: &mut Cursor<&[u8]>) -> Result<(), CmdError> {
        debug!("check");
        match get_u8(src)? {
            b'r' => {
                debug!("u8 read r command");
                let line = get_line(src)?;
                let _linestr = match String::from_utf8(line.to_vec()) {
                    Ok(v) => v,
                    Err(e) => panic!("invalid utf-8 syntax in read cmd: {:?}", e),
                };
                Ok(())
            }
            b'w' => {
                debug!("u8 read w command");
                get_line(src)?;
                Ok(())
            }
            b'O' => {
                debug!("u8 read O response");
                get_line(src)?;
                Ok(())
            }
            b'G' => {
                debug!("u8 read G response");
                get_line(src)?;
                Ok(())
            }
            b'E' => {
                debug!("u8 read E response");
                get_line(src)?;
                Ok(())
            }
            other => {
                debug!("check - other = {}", other);
                Err(format!("protocol error, unexpected byte `{}`", other).into())
            }
        }
    }

    pub(crate) fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, CmdError> {
        debug!("parse");
        match get_u8(src)? {
            b'r' => {
                // TODO: DRY this out?
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                debug!("got read line off the wire: {}", string);
                Ok(Frame::Read(string))
            }
            b'w' => {
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                debug!("got write line off the wire: {}", string);
                let (key, val) = string.split_once(' ').unwrap();
                Ok(Frame::Write(String::from(key), String::from(val)))
            }
            b'O' => {
                // OK response
                Ok(Frame::Success)
            }
            b'G' => {
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                debug!("got read result line off the wire: {}", string);
                let (_, two) = string.split_once(' ').unwrap();
                Ok(Frame::Value(String::from(two)))
            }
            b'E' => {
                let line = get_line(src)?.to_vec();
                let string = String::from_utf8(line)?;
                debug!("got error line off the wire: {}", string);
                let (_, msg) = string.split_once(' ').unwrap();
                Ok(Frame::Error(String::from(msg)))
            }
            _ => unimplemented!("implement parse frame for this"),
        }
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, CmdError> {
    debug!("get_u8");
    if !src.has_remaining() {
        debug!("get_u8 has no remaining data, returning Incomplete.");
        return Err(CmdError::Incomplete);
    }
    debug!("get_u8 has_remaining()");
    Ok(src.get_u8())
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], CmdError> {
    debug!("get line");
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;

    debug!("start: {} end: {}", start, end);
    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            src.set_position((i + 2) as u64);
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(CmdError::Incomplete)
}
