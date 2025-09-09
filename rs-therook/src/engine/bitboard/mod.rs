mod _bit;
mod _debug;
mod _iterator;
mod _utils;

pub use _utils::*;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Bitboard(0)
    }

    pub fn is_some(&self) -> bool {
        self.0 != 0
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}

impl TryFrom<Bitboard> for u8 {
    type Error = String;

    fn try_from(bitboard: Bitboard) -> Result<Self, Self::Error> {
        let u64 = u64::from(bitboard);

        if bitboard.is_none() {
            return Err("Cannot convert empty Bitboard to u8".into());
        }

        if u64 & u64 - 1 != 0 {
            return Err("Cannot convert Bitboard with multiple u8s into one u8".into());
        }

        Ok(u64.trailing_zeros() as u8)
    }
}

impl From<u64> for Bitboard {
    fn from(u64: u64) -> Self {
        Bitboard(u64)
    }
}

impl From<u8> for Bitboard {
    fn from(u8: u8) -> Self {
        Bitboard(1 << u8)
    }
}
