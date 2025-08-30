use super::*;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

impl Into<u64> for Bitboard {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}

impl<T: Into<u64>> BitAnd<T> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: T) -> Self::Output {
        Bitboard(self.0 & rhs.into())
    }
}

impl<T: Into<u64>> BitOr<T> for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: T) -> Self::Output {
        Bitboard(self.0 | rhs.into())
    }
}

impl<T: Into<u64>> BitXor<T> for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: T) -> Self::Output {
        Bitboard(self.0 ^ rhs.into())
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl<T: Into<u64>> Shl<T> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        Bitboard(self.0 << rhs.into())
    }
}

impl<T: Into<u64>> Shr<T> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: T) -> Self::Output {
        Bitboard(self.0 >> rhs.into())
    }
}
