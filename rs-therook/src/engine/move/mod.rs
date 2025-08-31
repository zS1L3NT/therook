mod _debug;

use super::*;

#[derive(PartialEq, Eq)]
pub struct Move(u16);

impl Move {
    pub const START_MASK: u16 = 0b111111 << 10;
    pub const END_MASK: u16 = 0b111111 << 4;
    pub const FLAG_MASK: u16 = 0b1111;

    pub const NO_FLAG: u8 = 0;
    pub const EN_PASSANT_FLAG: u8 = 1;
    pub const CASTLE: u8 = 2;
    pub const PAWN_DASH: u8 = 3;
    pub const PROMOTE_QUEEN: u8 = 4;
    pub const PROMOTE_ROOK: u8 = 5;
    pub const PROMOTE_BISHOP: u8 = 6;
    pub const PROMOTE_KNIGHT: u8 = 7;

    pub fn from(r#move: u16) -> Self {
        Move(r#move)
    }

    pub fn new(start: Tile, end: Tile, flag: u8) -> Self {
        Move(Into::<u16>::into(start) << 10 | Into::<u16>::into(end) << 4 | flag as u16)
    }

    pub fn get_start(&self) -> Tile {
        ((self.0 & Self::START_MASK) >> 10).into()
    }

    pub fn get_end(&self) -> Tile {
        ((self.0 & Self::END_MASK) >> 4).into()
    }

    pub fn get_flag(&self) -> u8 {
        (self.0 & Self::FLAG_MASK) as u8
    }

    pub fn get_promote_piece_type(&self) -> Option<u8> {
        match self.get_flag() {
            Self::PROMOTE_QUEEN => Some(Piece::QUEEN),
            Self::PROMOTE_ROOK => Some(Piece::ROOK),
            Self::PROMOTE_BISHOP => Some(Piece::BISHOP),
            Self::PROMOTE_KNIGHT => Some(Piece::KNIGHT),
            _ => None,
        }
    }
}

impl Into<u16> for Tile {
    fn into(self) -> u16 {
        self.get_value() as u16
    }
}

impl From<u16> for Tile {
    fn from(value: u16) -> Self {
        Tile::from(value as u8)
    }
}
