use PieceColor::*;

const WHITE: u8 = 1;
const BLACK: u8 = 0;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

impl From<PieceColor> for char {
    fn from(color: PieceColor) -> Self {
        match color {
            White => 'w',
            Black => 'b',
        }
    }
}

impl From<PieceColor> for u8 {
    fn from(color: PieceColor) -> Self {
        color as u8
    }
}

impl From<u8> for PieceColor {
    fn from(u8: u8) -> Self {
        match u8 {
            WHITE => White,
            BLACK => Black,
            _ => panic!("Unknown color: {u8:?}"),
        }
    }
}
