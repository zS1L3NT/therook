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

#[derive(Clone)]
pub struct Board<'a> {
    // Pre-computed data
    pub computed: &'a Computed,

    // Core information
    pub turn: PieceColor,
    pub squares: [Option<Piece>; 64],

    // Calculated on the fly
    pub pieces: [Bitboard; 12],
    pub colors: [Bitboard; 2],
    pub rays: [Bitboard; 2],
    pub attacks: [Bitboard; 2],
    pub pin_lines: [Vec<Bitboard>; 2],
    pub check_state: [CheckState; 2],

    // For undoing and restoration of state
    pub states: Vec<BoardState>,
}

impl<'a> Board<'a> {
    pub fn new(computed: &'a Computed) -> Self {
        Board {
            computed,

            turn: PieceColor::White,
            squares: [None; 64],

            pieces: [Bitboard::new(); 12],
            colors: [Bitboard::new(); 2],
            rays: [Bitboard::new(); 2],
            attacks: [Bitboard::new(); 2],
            pin_lines: [vec![], vec![]],
            check_state: [CheckState::None; 2],

            states: vec![],
        }
    }

    pub fn initial(computed: &'a Computed) -> Self {
        Board::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            computed,
        )
    }

    pub fn get_state(&self) -> &BoardState {
        self.states
            .last()
            .unwrap_or_else(|| panic!("No board state..."))
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
