mod _bit;
mod direction;

use super::*;
pub use direction::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Castling(u8);

impl Castling {
    pub fn new() -> Self {
        Castling(0)
    }

    pub fn initial() -> Self {
        Castling(
            CastlingDirection::WhiteKing as u8
                | CastlingDirection::WhiteQueen as u8
                | CastlingDirection::BlackKing as u8
                | CastlingDirection::BlackQueen as u8,
        )
    }

    pub fn can(&self, direction: CastlingDirection) -> bool {
        self.0 & direction as u8 != 0
    }
}

impl From<Castling> for u8 {
    fn from(castling: Castling) -> Self {
        castling.0
    }
}
