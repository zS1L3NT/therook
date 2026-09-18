use super::*;

#[derive(Clone)]
pub struct BoardState {
    pub castling: [bool; 4],
    pub enpassant: Bitboard,
    pub halfmove: u8,
    pub fullmove: u8,

    pub captured: Option<Piece>,

    // Derived board data from immediately before the move that produced this
    // state. Keeping it here lets undo restore the cache instead of rebuilding
    // attacks and pins for every piece a second time.
    pub previous_attacks: [Bitboard; 2],
    pub previous_pin_lines: [Vec<Bitboard>; 2],
    pub previous_check_state: [CheckState; 2],
}

impl BoardState {
    pub fn next(&self) -> Self {
        Self {
            castling: self.castling,
            enpassant: self.enpassant,
            halfmove: self.halfmove,
            fullmove: self.fullmove,
            captured: None,
            previous_attacks: [Bitboard::new(); 2],
            previous_pin_lines: [vec![], vec![]],
            previous_check_state: [CheckState::None; 2],
        }
    }

    pub fn new() -> Self {
        BoardState {
            castling: [false; 4],
            enpassant: Bitboard::new(),
            halfmove: 0,
            fullmove: 1,

            captured: None,

            previous_attacks: [Bitboard::new(); 2],
            previous_pin_lines: [vec![], vec![]],
            previous_check_state: [CheckState::None; 2],
        }
    }
}
