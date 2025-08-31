use PieceColor::*;

const WHITE: u8 = 1;
const BLACK: u8 = 0;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum PieceColor {
    White = WHITE,
    Black = BLACK,
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            White => Black,
            Black => White,
        }
    }
}

impl Into<char> for PieceColor {
    fn into(self) -> char {
        match self {
            White => 'w',
            Black => 'b',
        }
    }
}

impl Into<u8> for PieceColor {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for PieceColor {
    fn from(value: u8) -> Self {
        match value {
            WHITE => White,
            BLACK => Black,
            _ => panic!("Unknown team"),
        }
    }
}
