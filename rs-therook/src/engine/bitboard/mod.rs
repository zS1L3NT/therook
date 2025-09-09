mod _bit;
mod _debug;
mod _iterator;
mod _utils;

pub use _utils::*;

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
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

impl From<u64> for Bitboard {
    fn from(u64: u64) -> Self {
        Bitboard(u64)
    }
}

impl From<Tile> for Bitboard {
    fn from(tile: Tile) -> Self {
        Bitboard(1 << u8::from(tile))
    }
}

impl From<u8> for Bitboard {
    fn from(u8: u8) -> Self {
        Bitboard(1 << u8)
    }
}
