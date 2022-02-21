use std::io;

use bytes::{Buf, Bytes, BytesMut};
use rand::{thread_rng, RngCore};

use crate::error::CodecError;

pub const MAX_FRAME_LEN: usize = 65536;
pub const HEADER_LEN: usize = 4;
pub const NONCE_LEN: usize = 8;

// FIXME: Finish Frame logic and write header / message parser

#[derive(Debug)]
pub struct Frame {
    header: Header,
    #[allow(dead_code)]
    message: Message,
}

#[derive(Debug)]
pub struct Header {
    /// `u8` protocol version and direction of frame (request / response)
    pub version: u8,
    /// `u8` opcode that maps to a known command
    pub opcode: u8,
    /// `u16` field to determine length of the body
    pub length: u16,
}

#[derive(Debug)]
pub struct Message {
    /// `u64` randomly generated bytes that are used as a unique identifier
    pub nonce: u64,
    /// Variable length payload determined by the `length_field` value
    pub body: BytesMut,
}

impl Frame {
    pub fn parse_header(&self, mut src: BytesMut) -> Result<&Header, CodecError> {
        if src.is_empty() {
            return Err(CodecError::MissingData);
        }

        src.advance(2);

        Ok(&self.header)
    }
}

#[derive(Debug)]
pub enum Version {
    Request,
    Response,
}

impl TryFrom<u8> for Version {
    type Error = CodecError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Version::Request),
            0x81 => Ok(Version::Response),
            _ => Err(CodecError::IOError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "version field contains invalid data",
            ))),
        }
    }
}

#[derive(Debug)]
pub struct Nonce(Bytes);

impl Nonce {
    /// Constructs a new `Nonce` by generating a random array of bytes.
    pub fn new() -> Self {
        let mut buf = BytesMut::with_capacity(8);
        thread_rng().fill_bytes(&mut buf);
        Self(buf.freeze())
    }

    /// Get a reference to the inner data
    pub fn get_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Default for Nonce {
    fn default() -> Self {
        Self::new()
    }
}
