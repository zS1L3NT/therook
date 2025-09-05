mod _bit;
mod _debug;
mod _pieces;
mod color;
mod r#type;

pub use _pieces::*;
pub use color::*;
pub use r#type::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece(u8);

impl Piece {
    pub const COLOR_MASK: u8 = 1 << 5;
    pub const TYPE_MASK: u8 = !(1 << 5);
    pub const KING_MASK: u8 = 1 << 4;
    pub const ORTHOGONAL_MASK: u8 = 1 << 3;
    pub const DIAGONAL_MASK: u8 = 1 << 2;
    pub const KNIGHT_MASK: u8 = 1 << 1;
    pub const PAWN_MASK: u8 = 1 << 0;

    pub const ALL: [Piece; 12] = [
        WHITE_KING,
        WHITE_QUEEN,
        WHITE_ROOK,
        WHITE_BISHOP,
        WHITE_KNIGHT,
        WHITE_PAWN,
        BLACK_KING,
        BLACK_QUEEN,
        BLACK_ROOK,
        BLACK_BISHOP,
        BLACK_KNIGHT,
        BLACK_PAWN,
    ];

    pub fn get_color(&self) -> PieceColor {
        ((self.0 & Self::COLOR_MASK) >> 5).into()
    }

    pub fn get_type(&self) -> PieceType {
        (self.0 & Self::TYPE_MASK).into()
    }

    pub fn is_orthogonal_slider(&self) -> bool {
        (self.0 & Self::ORTHOGONAL_MASK) != 0
    }

    pub fn is_diagonal_slider(&self) -> bool {
        (self.0 & Self::DIAGONAL_MASK) != 0
    }

    pub fn is_slider(&self) -> bool {
        (self.0 & (Self::ORTHOGONAL_MASK | Self::DIAGONAL_MASK)) != 0
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        match piece {
            WHITE_KING => '\u{265A}',
            WHITE_QUEEN => '\u{265B}',
            WHITE_ROOK => '\u{265C}',
            WHITE_BISHOP => '\u{265D}',
            WHITE_KNIGHT => '\u{265E}',
            WHITE_PAWN => '\u{265F}',
            BLACK_KING => '\u{2654}',
            BLACK_QUEEN => '\u{2655}',
            BLACK_ROOK => '\u{2656}',
            BLACK_BISHOP => '\u{2657}',
            BLACK_KNIGHT => '\u{2658}',
            BLACK_PAWN => '\u{2659}',
            _ => panic!("Invalid piece"),
        }
    }
}
