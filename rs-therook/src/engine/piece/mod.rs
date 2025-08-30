mod _index;

pub use Piece::*;

use super::*;

#[repr(u8)]
pub enum Piece {
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}

impl Piece {
    pub const ALL: [Self; 12] = [
        WhiteKing,
        WhiteQueen,
        WhiteRook,
        WhiteBishop,
        WhiteKnight,
        WhitePawn,
        BlackKing,
        BlackQueen,
        BlackRook,
        BlackBishop,
        BlackKnight,
        BlackPawn,
    ];

    pub fn symbol(&self) -> char {
        match self {
            WhiteKing => '\u{2654}',
            WhiteQueen => '\u{2655}',
            WhiteRook => '\u{2656}',
            WhiteBishop => '\u{2657}',
            WhiteKnight => '\u{2658}',
            WhitePawn => '\u{2659}',
            BlackKing => '\u{265A}',
            BlackQueen => '\u{265B}',
            BlackRook => '\u{265C}',
            BlackBishop => '\u{265D}',
            BlackKnight => '\u{265E}',
            BlackPawn => '\u{265F}',
        }
    }
}
