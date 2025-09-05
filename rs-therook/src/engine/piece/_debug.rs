use super::*;

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            match *self {
                WHITE_KING => "WhiteKing",
                WHITE_QUEEN => "WhiteQueen",
                WHITE_ROOK => "WhiteRook",
                WHITE_BISHOP => "WhiteBishop",
                WHITE_KNIGHT => "WhiteKnight",
                WHITE_PAWN => "WhitePawn",
                BLACK_KING => "BlackKing",
                BLACK_QUEEN => "BlackQueen",
                BLACK_ROOK => "BlackRook",
                BLACK_BISHOP => "BlackBishop",
                BLACK_KNIGHT => "BlackKnight",
                BLACK_PAWN => "BlackPawn",
                _ => panic!("Unknown piece"),
            },
            char::from(*self)
        )
    }
}
