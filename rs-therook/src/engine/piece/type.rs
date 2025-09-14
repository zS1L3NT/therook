use PieceType::*;

const KING: u8 = 1 << 4;
const QUEEN: u8 = 1 << 3 | 1 << 2;
const ROOK: u8 = 1 << 3;
const BISHOP: u8 = 1 << 2;
const KNIGHT: u8 = 1 << 1;
const PAWN: u8 = 1;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King = KING,
    Queen = QUEEN,
    Rook = ROOK,
    Bishop = BISHOP,
    Knight = KNIGHT,
    Pawn = PAWN,
}

impl PieceType {
    pub const ALL: [PieceType; 6] = [King, Queen, Rook, Bishop, Knight, Pawn];
    pub const SLIDERS: [PieceType; 3] = [Queen, Rook, Bishop];

    pub fn is_orthogonal_slider(self) -> bool {
        self == Queen || self == Rook
    }

    pub fn is_diagonal_slider(self) -> bool {
        self == Queen || self == Bishop
    }

    pub fn is_slider(self) -> bool {
        self == Queen || self == Rook || self == Bishop
    }
}

impl From<PieceType> for u8 {
    fn from(r#type: PieceType) -> Self {
        r#type as u8
    }
}

impl From<u8> for PieceType {
    fn from(u8: u8) -> Self {
        match u8 {
            KING => King,
            QUEEN => Queen,
            ROOK => Rook,
            BISHOP => Bishop,
            KNIGHT => Knight,
            PAWN => Pawn,
            _ => panic!("Unknown piece type: {u8:?}"),
        }
    }
}
