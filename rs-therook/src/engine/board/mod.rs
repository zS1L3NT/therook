mod _calculate_moves;
mod _debug;
mod _index;
mod _make_move;
mod _undo_move;
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
    pub rays: [Bitboard; 2],
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
            rays: [Bitboard::new(); 2],
            captured: None,
        }
    }

    pub fn initial() -> Self {
        Fen::initial().try_into().unwrap()
    }

    pub fn update_rays(&mut self, color: PieceColor) {
        let line_masks = &self.computed.line_masks;

        self.rays[color] = Bitboard::new();

        for r#type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop] {
            for tile in self.pieces[color | r#type].get_tiles() {
                let index = u8::from(tile) as usize;

                if r#type.is_orthogonal_slider() {
                    self.rays[color] |= line_masks.ranks[index] | line_masks.files[index];
                }

                if r#type.is_diagonal_slider() {
                    self.rays[color] |= line_masks.diagonals[index] | line_masks.antidiags[index];
                }
            }
        }
    }

    pub fn set_tile(&mut self, tile: Tile, piece: Piece) {
        let bitboard = Bitboard::from(tile);
        let color = piece.get_color();

        self.squares[tile] = Some(piece);
        self.pieces[piece] |= bitboard;
        self.colors[color] |= bitboard;

        if piece.is_slider() {
            self.update_rays(color);
        }
    }

    pub fn clear_tile(&mut self, tile: Tile, piece: Piece) {
        let bitboard = Bitboard::from(tile);
        let color = piece.get_color();

        self.squares[tile] = None;
        self.pieces[piece] ^= bitboard;
        self.colors[color] ^= bitboard;

        if piece.is_slider() {
            self.update_rays(color);
        }
    }

    pub fn move_piece(&mut self, from: Tile, to: Tile, piece: Piece) {
        let bitboard = Bitboard::from(from) | Bitboard::from(to);
        let color = piece.get_color();

        self.squares[from] = None;
        self.squares[to] = Some(piece);
        self.pieces[piece] ^= bitboard;
        self.colors[color] ^= bitboard;

        if piece.is_slider() {
            self.update_rays(color);
        }
    }
}
