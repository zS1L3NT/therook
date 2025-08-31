mod _debug;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(u16);

impl Move {
    pub const START_MASK: u16 = 0b111111 << 10;
    pub const END_MASK: u16 = 0b111111 << 4;
    pub const FLAG_MASK: u16 = 0b1111;

    pub const NO_FLAG: u16 = 0;
    pub const EN_PASSANT_FLAG: u16 = 1;
    pub const CASTLE: u16 = 2;
    pub const PAWN_DASH: u16 = 3;
    pub const PROMOTE_QUEEN: u16 = 4;
    pub const PROMOTE_ROOK: u16 = 5;
    pub const PROMOTE_BISHOP: u16 = 6;
    pub const PROMOTE_KNIGHT: u16 = 7;

    pub fn from(r#move: u16) -> Self {
        Move(r#move)
    }

    pub fn new(start: u8, end: u8, flag: u8) -> Self {
        Move((start as u16) << 10 | (end as u16) << 4 | flag as u16)
    }

    pub fn get_start(&self) -> u8 {
        ((self.0 & Self::START_MASK) >> 10) as u8
    }

    pub fn get_end(&self) -> u8 {
        ((self.0 & Self::END_MASK) >> 4) as u8
    }

    pub fn get_flag(&self) -> u8 {
        (self.0 & Self::FLAG_MASK) as u8
    }

    pub fn get_promote_piece_type(&self) -> Option<u8> {
        match self.get_flag() as u16 {
            Self::PROMOTE_QUEEN => Some(Piece::QUEEN),
            Self::PROMOTE_ROOK => Some(Piece::ROOK),
            Self::PROMOTE_BISHOP => Some(Piece::BISHOP),
            Self::PROMOTE_KNIGHT => Some(Piece::KNIGHT),
            _ => None,
        }
    }
}
