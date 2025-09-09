use super::*;
use CastlingDirection::*;

pub enum CastlingDirection {
    WhiteKing = 1 << 3,
    WhiteQueen = 1 << 2,
    BlackKing = 1 << 1,
    BlackQueen = 1 << 0,
}

impl From<Piece> for CastlingDirection {
    fn from(piece: Piece) -> Self {
        match piece {
            WHITE_KING => WhiteKing,
            WHITE_QUEEN => WhiteQueen,
            BLACK_KING => BlackKing,
            BLACK_QUEEN => BlackKing,
            _ => unreachable!(),
        }
    }
}
