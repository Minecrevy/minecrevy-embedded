#![no_std]

mod error;
mod ext_async;
mod ext_blocking;
mod io_async;
mod io_blocking;
pub mod options;
mod size;

use heapless::Vec;

pub use self::{error::*, ext_async::*, ext_blocking::*, io_async::*, io_blocking::*, size::*};

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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RawPacket<const N: usize> {
    pub id: i32,
    pub data: Vec<u8, N>,
}
