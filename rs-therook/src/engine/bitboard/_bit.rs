use super::*;
use std::ops::*;

impl BitAnd<Bitboard> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 & rhs)
    }
}

impl BitAnd<u8> for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 & (1 << rhs))
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOr<u64> for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 | rhs)
    }
}

impl BitOr<u8> for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 | (1 << rhs))
    }
}

impl BitXor<Bitboard> for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXor<u64> for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 ^ rhs)
    }
}

impl BitXor<u8> for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 ^ (1 << rhs))
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0
    }
}

impl BitAndAssign<u64> for Bitboard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs
    }
}

impl BitAndAssign<u8> for Bitboard {
    fn bitand_assign(&mut self, rhs: u8) {
        self.0 &= 1 << rhs
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0
    }
}

impl BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs
    }
}

impl BitOrAssign<u8> for Bitboard {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= 1 << rhs
    }
}

impl BitXorAssign<Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0
    }
}

impl BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs
    }
}

impl BitXorAssign<u8> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u8) {
        self.0 ^= 1 << rhs
    }
}

impl ShlAssign<u8> for Bitboard {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}

impl ShrAssign<u8> for Bitboard {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}
