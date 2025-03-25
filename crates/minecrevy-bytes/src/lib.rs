#![no_std]

mod io_async;
mod io_blocking;

use embedded_byteorder::ReadExactError;
use heapless::Vec;
use thiserror::Error;

pub use self::io_async::*;
pub use self::io_blocking::*;

pub const fn var_i32_size(value: i32) -> usize {
    static VAR_INT_LENGTHS: [usize; 33] = const {
        let mut lengths = [0; 33];
        let mut i: usize = 0;
        while i <= 32 {
            let sub = match i.checked_sub(1) {
                Some(x) => x,
                None => 0,
            };
            lengths[i] = (31 - sub).div_ceil(7);
            i += 1;
        }
        lengths[32] = 1; // Special case for the number 0.
        lengths
    };

    VAR_INT_LENGTHS[value.leading_zeros() as usize]
}

#[derive(Error, Clone, PartialEq, Eq, Debug)]
pub enum ReadMinecraftError<E> {
    #[error("invalid UTF-8")]
    InvalidUtf8,
    #[error("length exceeded")]
    LengthExceeded,
    #[error("varint too big")]
    VarIntTooBig,
    #[error("unexpected EOF")]
    UnexpectedEof,
    #[error("other error: {0}")]
    Other(#[from] E),
}

impl<E> From<ReadExactError<E>> for ReadMinecraftError<E> {
    fn from(value: ReadExactError<E>) -> Self {
        match value {
            ReadExactError::UnexpectedEof => ReadMinecraftError::UnexpectedEof,
            ReadExactError::Other(e) => ReadMinecraftError::Other(e),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RawPacket<const N: usize> {
    pub id: i32,
    pub data: Vec<u8, N>,
}
