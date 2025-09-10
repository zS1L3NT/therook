mod _calculate_moves;
mod _debug;
mod _index;
mod _make_move;
mod _undo_move;
mod _update;
mod check_state;

use super::*;
use crate::interfaces::*;
pub use check_state::*;

pub struct Board {
    // From FEN
    pub squares: [Option<Piece>; 64],
    pub turn: PieceColor,
    pub castling: [bool; 4],
    pub enpassant: Bitboard,
    pub halfmove: u8,
    pub fullmove: u8,

    // Pre-computed data
    pub computed: Computed,

    // Extra state of the board
    pub pieces: [Bitboard; 12],
    pub colors: [Bitboard; 2],
    pub rays: [Bitboard; 2],
    pub attacks: [Bitboard; 2],
    pub pin_lines: [Bitboard; 2],
    pub check_state: [CheckState; 2],
    pub captured: Option<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [None; 64],
            turn: PieceColor::White,
            castling: [false; 4],
            enpassant: Bitboard::new(),
            halfmove: 0,
            fullmove: 1,

            computed: Computed::new(),

            pieces: [Bitboard::new(); 12],
            colors: [Bitboard::new(); 2],
            rays: [Bitboard::new(); 2],
            attacks: [Bitboard::new(); 2],
            check_state: [CheckState::None; 2],
            pin_lines: [Bitboard::new(); 2],
            captured: None,
        }
    }

    pub fn fen(string: String) -> Result<Self, FenError> {
        Fen::new(string).try_into()
    }

    pub fn initial() -> Self {
        Fen::initial().try_into().unwrap()
    }

    pub fn set_square(&mut self, square: u8, piece: Piece) {
        let bitboard = Bitboard::from(square);
        let color = piece.get_color();

        self.squares[square as usize] = Some(piece);
        self.pieces[piece] |= bitboard;
        self.colors[color] |= bitboard;

        if piece.is_slider() {
            self.update_rays(color);
        }
        // self.update_pin_lines(color);
        // self.update_pin_lines(color.opposite());
    }

    pub fn clear_square(&mut self, square: u8, piece: Piece) {
        let bitboard = Bitboard::from(square);
        let color = piece.get_color();

        self.squares[square as usize] = None;
        self.pieces[piece] ^= bitboard;
        self.colors[color] ^= bitboard;

        if piece.is_slider() {
            self.update_rays(color);
        }
        // self.update_pin_lines(color);
        // self.update_pin_lines(color.opposite());
    }

    pub fn move_piece(&mut self, from_square: u8, to_square: u8, piece: Piece) {
        let bitboard = Bitboard::from(from_square) | Bitboard::from(to_square);
        let color = piece.get_color();

        self.squares[from_square as usize] = None;
        self.squares[to_square as usize] = Some(piece);
        self.pieces[piece] ^= bitboard;
        self.colors[color] ^= bitboard;

        if piece.is_slider() {
            self.update_rays(color);
        }
        // self.update_pin_lines(color);
        // self.update_pin_lines(color.opposite());
    }
}
