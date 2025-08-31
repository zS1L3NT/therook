use PieceType::*;

const KING: u8 = 1 << 4;
const QUEEN: u8 = 1 << 3 | 1 << 2;
const ROOK: u8 = 1 << 3;
const BISHOP: u8 = 1 << 2;
const KNIGHT: u8 = 1 << 1;
const PAWN: u8 = 1;

#[repr(u8)]
pub enum PieceType {
    King = KING,
    Queen = QUEEN,
    Rook = ROOK,
    Bishop = BISHOP,
    Knight = KNIGHT,
    Pawn = PAWN,
}

impl Into<u8> for PieceType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for PieceType {
    fn from(value: u8) -> Self {
        match value {
            KING => King,
            QUEEN => Queen,
            ROOK => Rook,
            BISHOP => Bishop,
            KNIGHT => Knight,
            PAWN => Pawn,
            _ => panic!("Unknown piece type"),
        }
    }
}
