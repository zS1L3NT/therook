mod _bit;
mod direction;

use super::*;
pub use direction::*;

pub struct Castling(u8);

impl Castling {
    pub fn new() -> Self {
        Castling(0)
    }

    pub fn initial() -> Self {
        Castling(WhiteKing as u8 | WhiteQueen as u8 | BlackKing as u8 | BlackQueen as u8)
    }

    // Representation of CastlingDirection
    pub fn can(&self, direction: CastlingDirection) -> bool {
        self.0 & direction as u8 != 0
    }
}

impl From<&Castling> for u8 {
    fn from(castling: &Castling) -> Self {
        castling.0
    }
}
