mod _debug;
mod _make_move;
mod _moves;
mod _undo_move;
mod castling;

use super::*;
use castling::*;
use therook::bitboard;

pub struct Board {
    // From FEN
    pub bitboards: [Bitboard; 12],
    pub turn: Team,
    pub castling: Castling,
    pub enpassant: Bitboard,
    pub halfmove: u8,
    pub fullmove: u8,

    // Extra state of the board
    pub squares: [Option<Piece>; 64],
    pub captured: Option<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            bitboards: [Bitboard::new(); 12],
            turn: Team::White,
            castling: Castling::new(),
            enpassant: Bitboard::new(),
            halfmove: 0,
            fullmove: 1,

            squares: [None; 64],
            captured: None,
        }
    }

    pub fn initial() -> Self {
        let mut board = Board::new();

        board.bitboards[WHITE_KING] = bitboard!(E1);
        board.bitboards[WHITE_QUEEN] = bitboard!(D1);
        board.bitboards[WHITE_ROOK] = bitboard!(A1) | bitboard!(H1);
        board.bitboards[WHITE_BISHOP] = bitboard!(B1) | bitboard!(G1);
        board.bitboards[WHITE_KNIGHT] = bitboard!(C1) | bitboard!(F1);
        board.bitboards[WHITE_PAWN] = RANK_2;
        board.bitboards[BLACK_KING] = bitboard!(E8);
        board.bitboards[BLACK_QUEEN] = bitboard!(D8);
        board.bitboards[BLACK_ROOK] = bitboard!(A8) | bitboard!(H8);
        board.bitboards[BLACK_BISHOP] = bitboard!(B8) | bitboard!(G8);
        board.bitboards[BLACK_KNIGHT] = bitboard!(C8) | bitboard!(F8);
        board.bitboards[BLACK_PAWN] = RANK_7;

        board.castling = Castling::initial();

        board
    }
}
