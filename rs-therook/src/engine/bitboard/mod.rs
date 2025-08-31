mod _bitops;
mod _debug;
mod _rank_file;

pub use _rank_file::*;

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Bitboard(0)
    }

    pub fn from(bitboard: u64) -> Self {
        Bitboard(bitboard)
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

        return tiles;
    }
}

impl Into<Tile> for Bitboard {
    fn into(self) -> Tile {
        if self.is_empty() {
            println!("{:?}", self);
            panic!("Cannot call Bitboard::get_tile(&self) when bitboard is empty!");
        }

        if self.0 & self.0 - 1 != 0 {
            println!("{:?}", self);
            panic!("Cannot call Bitboard::get_tile(&self) when bitboard has more than one bit!");
        }

        return Tile::from(self.0.trailing_zeros() as u8);
    }
}
