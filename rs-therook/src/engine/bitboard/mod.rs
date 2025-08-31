mod _bitops;
mod _debug;
mod _tiles;

pub use _tiles::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn new() -> Self {
        Bitboard(0)
    }

    pub fn from(bitboard: u64) -> Self {
        Bitboard(bitboard)
    }

    pub fn value(self) -> u64 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn get_indexes(&self) -> Vec<u8> {
        let mut indexes: Vec<u8> = vec![];
        let mut bitboard = self.0;

        while bitboard != 0 {
            indexes.push(bitboard.trailing_zeros() as u8);
            bitboard &= bitboard - 1;
        }

        return indexes;
    }
}
