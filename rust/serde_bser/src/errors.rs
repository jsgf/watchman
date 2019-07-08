use std::fmt;

use serde::{de, ser};
use snafu::Snafu;

pub use snafu::ResultExt;
pub type Result<T> = std::result::Result<T, Error>;

use crate::header::HeaderByte;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("while deserializing BSER: invalid state byte for {}: {}", kind, byte))]
    DeInvalidStartByte { kind: String, byte: HeaderByte },
    #[snafu(display("error while deserializing BSER: {}", msg))]
    DeCustom { msg: String },
    #[snafu(display("while deserializing BSER: recursion limit exceeded with {}", kind))]
    DeRecursionLimitExceeded { kind: String },
    #[snafu(display("error while serializing BSER: {}", msg))]
    SerCustom { msg: String },
    #[snafu(display("while serializing BSER: need size of {}", kind))]
    SerNeedSize { kind: &'static str },
    #[snafu(display("while serializing BSER: integer too big: {}", v))]
    SerU64TooBig { v: u64 },
    #[snafu(display("IO Error: {}", source))]
    Io { source: std::io::Error },
    #[snafu(display("UTF8 error: {}", source))]
    Utf8 { source: std::str::Utf8Error },
    #[snafu(display("Error {}: {}", msg, source))]
    ErrMsg {
        msg: String,
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io { source: err }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Utf8 { source: err }
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::DeCustom {
            msg: format!("{}", msg),
        }
        .into()
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::SerCustom {
            msg: format!("{}", msg),
        }
        .into()
    }
}
