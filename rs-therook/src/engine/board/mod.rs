mod _calculate_moves;
mod _debug;
mod _index;
mod _make_move;
mod _undo_move;
mod _update;
mod check_state;
mod state;

use super::*;
pub use check_state::*;
pub use state::*;

pub struct Board {
    // Pre-computed data
    pub computed: Computed,

    // Core information
    pub turn: PieceColor,
    pub squares: [Option<Piece>; 64],

    // Calculated on the fly
    pub pieces: [Bitboard; 12],
    pub colors: [Bitboard; 2],
    pub rays: [Bitboard; 2],
    pub attacks: [Bitboard; 2],
    pub pin_lines: [Bitboard; 2],
    pub check_state: [CheckState; 2],

    // For undoing and restoration of state
    pub states: Vec<BoardState>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            computed: Computed::new(),

            turn: PieceColor::White,
            squares: [None; 64],

            pieces: [Bitboard::new(); 12],
            colors: [Bitboard::new(); 2],
            rays: [Bitboard::new(); 2],
            attacks: [Bitboard::new(); 2],
            pin_lines: [Bitboard::new(); 2],
            check_state: [CheckState::None; 2],

            states: vec![],
        }
    }

    pub fn get_state(&self) -> &BoardState {
        self.states
            .last()
            .unwrap_or_else(|| panic!("No board state..."))
    }

    pub fn initial() -> Self {
        "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
            .try_into()
            .unwrap()
    }

    pub fn set_square(&mut self, square: u8, piece: Piece) {
        let bitboard = Bitboard::from(square);
        let color = piece.get_color();

        self.squares[square as usize] = Some(piece);
        self.pieces[piece] |= bitboard;
        self.colors[color] |= bitboard;
    }

    pub fn clear_square(&mut self, square: u8, piece: Piece) {
        let bitboard = Bitboard::from(square);
        let color = piece.get_color();

        self.squares[square as usize] = None;
        self.pieces[piece] ^= bitboard;
        self.colors[color] ^= bitboard;
    }
}
