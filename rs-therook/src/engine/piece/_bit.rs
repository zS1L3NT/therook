use super::*;
use std::ops::*;

impl BitOr<PieceType> for PieceColor {
    type Output = Piece;
    fn bitor(self, rhs: PieceType) -> Piece {
        Piece(u8::from(self) << 5 | u8::from(rhs))
    }
}
