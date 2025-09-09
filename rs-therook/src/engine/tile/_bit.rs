use super::*;
use std::ops::*;

impl BitOr<Tile> for Tile {
    type Output = Bitboard;
    fn bitor(self, rhs: Tile) -> Self::Output {
        Bitboard::from((1u64 << self.0) | (1u64 << u8::from(rhs)))
    }
}

impl Not for Tile {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        !Bitboard::from(self.0)
    }
}

impl Shl<u8> for Tile {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Tile(self.0 + rhs)
    }
}

impl Shr<u8> for Tile {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Tile(self.0 - rhs)
    }
}

impl ShlAssign<u8> for Tile {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 += rhs
    }
}

impl ShrAssign<u8> for Tile {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 -= rhs
    }
}
