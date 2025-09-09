mod _bit;
mod _debug;
mod _utils;

pub use _utils::*;

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Bitboard(0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    
    pub fn is_some(&self) -> bool {
        self.0 != 0
    }
}

impl Iterator for Bitboard {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let index = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Some(index)
    }
}

impl From<Bitboard> for u64 {
    fn from(bitboard: Bitboard) -> Self {
        bitboard.0
    }
}

impl From<Tile> for Bitboard {
    fn from(tile: Tile) -> Self {
        Bitboard(1 << u8::from(tile))
    }
}

impl From<u64> for Bitboard {
    fn from(u64: u64) -> Self {
        Bitboard(u64)
    }
}
