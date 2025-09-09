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

impl TryFrom<Bitboard> for Tile {
    type Error = String;

    fn try_from(bitboard: Bitboard) -> Result<Self, Self::Error> {
        let u64 = u64::from(bitboard);

        if bitboard.is_none() {
            return Err("Cannot convert empty Bitboard to Tile".into());
        }

        if u64 & u64 - 1 != 0 {
            return Err("Cannot convert Bitboard with multiple Tiles into one Tile".into());
        }

        Ok(Tile::from(u64.trailing_zeros() as u8))
    }
}
