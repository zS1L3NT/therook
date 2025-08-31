mod _bit;
mod _debug;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile(u8);

impl Into<Bitboard> for Tile {
    fn into(self) -> Bitboard {
        Bitboard::from(1 << self.0)
    }
}

impl Into<u8> for Tile {
    fn into(self) -> u8 {
        self.0
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        Tile(value)
    }
}
