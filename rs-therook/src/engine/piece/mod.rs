mod _index;
mod _pieces;

pub use _pieces::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece(u8);

impl Piece {
    pub const TEAM_MASK: u8 = 0b100000;
    pub const TYPE_MASK: u8 = 0b011111;
    pub const KING_MASK: u8 = 0b010000;
    pub const ORTHOGONAL_MASK: u8 = 0b001000;
    pub const DIAGONAL_MASK: u8 = 0b000100;
    pub const KNIGHT_MASK: u8 = 0b000010;
    pub const PAWN_MASK: u8 = 0b000001;

    pub const WHITE: u8 = 1 << 5;
    pub const BLACK: u8 = 0;

    pub const KING: u8 = 1 << 4;
    pub const QUEEN: u8 = Self::BISHOP | Self::ROOK;
    pub const ROOK: u8 = 1 << 3;
    pub const BISHOP: u8 = 1 << 2;
    pub const KNIGHT: u8 = 1 << 1;
    pub const PAWN: u8 = 1;
    pub const NONE: u8 = 0;

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

    pub fn new(piece: u8) -> Self {
        Piece(piece)
    }

    pub fn get_team(&self) -> u8 {
        self.0 & Self::TEAM_MASK
    }

    pub fn get_type(&self) -> u8 {
        self.0 & Self::TYPE_MASK
    }

    pub fn is_orthogonal_slider(&self) -> bool {
        self.0 & Self::ORTHOGONAL_MASK != 0
    }

    pub fn is_diagonal_slider(&self) -> bool {
        self.0 & Self::DIAGONAL_MASK != 0
    }

    pub fn is_slider(&self) -> bool {
        self.0 & (Self::ORTHOGONAL_MASK | Self::DIAGONAL_MASK) != 0
    }

    pub fn symbol(&self) -> char {
        match *self {
            WHITE_KING => '\u{2654}',
            WHITE_QUEEN => '\u{2655}',
            WHITE_ROOK => '\u{2656}',
            WHITE_BISHOP => '\u{2657}',
            WHITE_KNIGHT => '\u{2658}',
            WHITE_PAWN => '\u{2659}',
            BLACK_KING => '\u{265A}',
            BLACK_QUEEN => '\u{265B}',
            BLACK_ROOK => '\u{265C}',
            BLACK_BISHOP => '\u{265D}',
            BLACK_KNIGHT => '\u{265E}',
            BLACK_PAWN => '\u{266F}',
            _ => panic!("Invalid piece type"),
        }
    }
}
