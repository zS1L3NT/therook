mod _calculate_moves;
mod _debug;
mod _index;
mod _make_move;
mod _undo_move;
mod _update_pin_lines;
mod castling;

use super::*;
use crate::interfaces::*;
pub use castling::*;

pub struct Board {
    // From FEN
    pub squares: [Option<Piece>; 64],
    pub turn: PieceColor,
    pub castling: Castling,
    pub enpassant: Bitboard,
    pub halfmove: u8,
    pub fullmove: u8,

    // Pre-computed data
    pub computed: Computed,

    // Extra state of the board
    pub pieces: [Bitboard; 12],
    pub colors: [Bitboard; 2],
    pub pin_lines: [Bitboard; 2],
    pub captured: Option<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            squares: [None; 64],
            turn: PieceColor::White,
            castling: Castling::new(),
            enpassant: Bitboard::new(),
            halfmove: 0,
            fullmove: 1,

            computed: Computed::new(),

            pieces: [Bitboard::new(); 12],
            colors: [Bitboard::new(); 2],
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

    pub fn set_tile(&mut self, tile: Tile, piece: Piece) {
        let bitboard = Bitboard::from(tile);
        let color = piece.get_color();

        self.squares[tile] = Some(piece);
        self.pieces[piece] |= bitboard;
        self.colors[color] |= bitboard;

        // self.update_pin_lines(color);
        // self.update_pin_lines(color.opposite());
    }

    pub fn clear_tile(&mut self, tile: Tile, piece: Piece) {
        let bitboard = Bitboard::from(tile);
        let color = piece.get_color();

        self.squares[tile] = None;
        self.pieces[piece] ^= bitboard;
        self.colors[color] ^= bitboard;

        // self.update_pin_lines(color);
        // self.update_pin_lines(color.opposite());
    }

    pub fn move_piece(&mut self, from: Tile, to: Tile, piece: Piece) {
        let bitboard = Bitboard::from(from) | Bitboard::from(to);
        let color = piece.get_color();

        self.squares[from] = None;
        self.squares[to] = Some(piece);
        self.pieces[piece] ^= bitboard;
        self.colors[color] ^= bitboard;

        // self.update_pin_lines(color);
        // self.update_pin_lines(color.opposite());
    }
}
