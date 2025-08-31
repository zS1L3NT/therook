mod _debug;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile(u8);

impl Tile {
    pub fn from(tile: u8) -> Self {
        Tile(tile)
    }

    pub fn get_value(self) -> u8 {
        self.0
    }
}

impl Into<Bitboard> for Tile {
    fn into(self) -> Bitboard {
        Bitboard::from(1 << self.0)
    }
}
