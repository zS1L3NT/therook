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

    pub fn get_tiles(&self) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = vec![];
        let mut bitboard = self.0;

        while bitboard != 0 {
            tiles.push(Tile::from(bitboard.trailing_zeros() as u8));
            bitboard &= bitboard - 1;
        }

        tiles
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
