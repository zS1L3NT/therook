use super::*;

#[derive(Clone)]
pub struct BoardState {
    pub castling: [bool; 4],
    pub enpassant: Bitboard,
    pub halfmove: u8,
    pub fullmove: u8,

    pub captured: Option<Piece>,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            castling: [false; 4],
            enpassant: Bitboard::new(),
            halfmove: 0,
            fullmove: 1,

            captured: None,
        }
    }
}
