mod _bit;
mod _debug;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile(u8);

impl From<Tile> for u8 {
    fn from(tile: Tile) -> Self {
        tile.0
    }
}

impl From<u8> for Tile {
    fn from(u8: u8) -> Self {
        Tile(u8)
    }
}

impl From<Bitboard> for Tile {
    fn from(bitboard: Bitboard) -> Tile {
        let u64 = u64::from(bitboard);

        if bitboard.is_empty() {
            println!("{:?}", bitboard);
            panic!("Cannot call Bitboard::get_tile(&self) when bitboard is empty!");
        }

        if u64 & u64 - 1 != 0 {
            println!("{:?}", bitboard);
            panic!("Cannot call Bitboard::get_tile(&self) when bitboard has more than one bit!");
        }

        Tile::from(u64.trailing_zeros() as u8)
    }
}
