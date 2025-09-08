use super::*;

impl std::ops::BitOr<CastlingDirection> for Castling {
    type Output = Castling;
    fn bitor(self, rhs: CastlingDirection) -> Self::Output {
        Castling(self.0 | (rhs as u8))
    }
}

impl std::ops::BitOrAssign<CastlingDirection> for Castling {
    fn bitor_assign(&mut self, rhs: CastlingDirection) {
        self.0 |= rhs as u8;
    }
}
