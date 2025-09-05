use super::*;
use std::ops::*;

impl<T: Into<Bitboard>> BitOr<T> for Tile {
    type Output = Bitboard;
    fn bitor(self, rhs: T) -> Self::Output {
        Bitboard::from(self) | rhs.into()
    }
}

impl Not for Tile {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        !Bitboard::from(self)
    }
}

impl<T: Into<u8>> Shl<T> for Tile {
    type Output = Tile;
    fn shl(self, rhs: T) -> Self::Output {
        (self.0 << rhs.into()).into()
    }
}

impl<T: Into<u8>> Shr<T> for Tile {
    type Output = Tile;
    fn shr(self, rhs: T) -> Self::Output {
        (self.0 >> rhs.into()).into()
    }
}

impl<T: Into<u8>> ShlAssign<T> for Tile {
    fn shl_assign(&mut self, rhs: T) {
        self.0 <<= rhs.into()
    }
}

impl<T: Into<u8>> ShrAssign<T> for Tile {
    fn shr_assign(&mut self, rhs: T) {
        self.0 >>= rhs.into()
    }
}
