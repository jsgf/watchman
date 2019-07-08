//! Header constants for BSER.

use std::fmt;

pub const EMPTY_HEADER: &[u8] = b"\x00\x02\x00\x00\x00\x00\x05\x00\x00\x00\x00";

pub const BSER_ARRAY: u8 = 0x00;
pub const BSER_OBJECT: u8 = 0x01;
pub const BSER_BYTESTRING: u8 = 0x02;
pub const BSER_INT8: u8 = 0x03;
pub const BSER_INT16: u8 = 0x04;
pub const BSER_INT32: u8 = 0x05;
pub const BSER_INT64: u8 = 0x06;
pub const BSER_REAL: u8 = 0x07;
pub const BSER_TRUE: u8 = 0x08;
pub const BSER_FALSE: u8 = 0x09;
pub const BSER_NULL: u8 = 0x0a;
pub const BSER_TEMPLATE: u8 = 0x0b;
pub const BSER_SKIP: u8 = 0x0c;
pub const BSER_UTF8STRING: u8 = 0x0d;

// Capabilities (we would ideally want to use EnumSet here, but
// https://github.com/contain-rs/enum-set/issues/21 stops us)
#[allow(unused)]
pub const BSER_CAP_DISABLE_UNICODE: u8 = 0x01;
#[allow(unused)]
pub const BSER_CAP_DISABLE_UNICODE_FOR_ERRORS: u8 = 0x02;

#[derive(Debug)]
pub struct HeaderByte(pub u8);

impl fmt::Display for HeaderByte {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self.0 {
            BSER_ARRAY => "BSER_ARRAY",
            BSER_OBJECT => "BSER_OBJECT",
            BSER_BYTESTRING => "BSER_BYTESTRING",
            BSER_INT8 => "BSER_INT8",
            BSER_INT16 => "BSER_INT16",
            BSER_INT32 => "BSER_INT32",
            BSER_INT64 => "BSER_INT64",
            BSER_REAL => "BSER_REAL",
            BSER_TRUE => "BSER_TRUE",
            BSER_FALSE => "BSER_FALSE",
            BSER_NULL => "BSER_NULL",
            BSER_TEMPLATE => "BSER_TEMPLATE",
            BSER_SKIP => "BSER_SKIP",
            BSER_UTF8STRING => "BSER_UTF8STRING",
            ch => return write!(fmt, "unknown byte '{:?}'", ch),
        };
        fmt.write_str(msg)
    }
}
