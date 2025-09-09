mod _debug;
mod flag;

use super::*;
pub use flag::*;

#[derive(PartialEq, Eq)]
pub struct Move(u16);

impl Move {
    pub const START_MASK: u16 = 0b111111 << 10;
    pub const END_MASK: u16 = 0b111111 << 4;
    pub const FLAG_MASK: u16 = 0b1111;

    pub fn new(start: Tile, end: Tile, flag: MoveFlag) -> Self {
        Move((u8::from(start) as u16) << 10 | (u8::from(end) as u16) << 4 | u8::from(flag) as u16)
    }

    pub fn get_start(&self) -> Tile {
        Tile::from(((self.0 & Self::START_MASK) >> 10) as u8)
    }

    pub fn get_end(&self) -> Tile {
        Tile::from(((self.0 & Self::END_MASK) >> 4) as u8)
    }

    pub fn get_flag(&self) -> MoveFlag {
        ((self.0 & Self::FLAG_MASK) as u8).into()
    }

    pub fn get_promote_piece_type(&self) -> Option<PieceType> {
        match self.get_flag() {
            MoveFlag::PromoteQueen => Some(PieceType::Queen),
            MoveFlag::PromoteRook => Some(PieceType::Rook),
            MoveFlag::PromoteBishop => Some(PieceType::Bishop),
            MoveFlag::PromoteKnight => Some(PieceType::Knight),
            _ => None,
        }
    }
}
