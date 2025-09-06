use super::*;

impl std::ops::BitOr<CastleDirection> for Castling {
    type Output = Castling;
    fn bitor(self, rhs: CastleDirection) -> Self::Output {
        Castling(self.0 | (rhs as u8))
    }
}

impl std::ops::BitOrAssign<CastleDirection> for Castling {
    fn bitor_assign(&mut self, rhs: CastleDirection) {
        self.0 |= rhs as u8;
    }
}
