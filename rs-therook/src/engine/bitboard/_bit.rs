use super::*;
use std::ops::*;

impl<T: Into<u64>> BitAnd<T> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: T) -> Self::Output {
        (self.0 & rhs.into()).into()
    }
}

impl<T: Into<u64>> BitOr<T> for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: T) -> Self::Output {
        (self.0 | rhs.into()).into()
    }
}

impl<T: Into<u64>> BitXor<T> for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: T) -> Self::Output {
        (self.0 ^ rhs.into()).into()
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        (!self.0).into()
    }
}

impl<T: Into<u64>> Shl<T> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        (self.0 << rhs.into()).into()
    }
}

impl<T: Into<u64>> Shr<T> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: T) -> Self::Output {
        (self.0 >> rhs.into()).into()
    }
}

impl<T: Into<u64>> BitAndAssign<T> for Bitboard {
    fn bitand_assign(&mut self, rhs: T) {
        self.0 &= rhs.into();
    }
}

impl<T: Into<u64>> BitOrAssign<T> for Bitboard {
    fn bitor_assign(&mut self, rhs: T) {
        self.0 |= rhs.into();
    }
}

impl<T: Into<u64>> BitXorAssign<T> for Bitboard {
    fn bitxor_assign(&mut self, rhs: T) {
        self.0 ^= rhs.into();
    }
}

impl<T: Into<u64>> ShlAssign<T> for Bitboard {
    fn shl_assign(&mut self, rhs: T) {
        self.0 <<= rhs.into();
    }
}

impl<T: Into<u64>> ShrAssign<T> for Bitboard {
    fn shr_assign(&mut self, rhs: T) {
        self.0 >>= rhs.into();
    }
}
