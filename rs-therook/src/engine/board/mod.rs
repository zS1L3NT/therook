mod _debug;
mod _moves;
mod castling;

use super::*;
use castling::*;
use therook::square;

pub struct Board {
    // From FEN
    pub bitboards: [Bitboard; 12],
    pub turn: Team,
    pub castling: Castling,
    pub enpassant: Bitboard,
    pub halfmove: u8,
    pub fullmove: u8,

    // Extra state of the board
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

            captured: None,
        }
    }

    pub fn initial() -> Self {
        let mut board = Board::new();

        board.bitboards[WHITE_KING] = square!(E1);
        board.bitboards[WHITE_QUEEN] = square!(D1);
        board.bitboards[WHITE_ROOK] = square!(A1) | square!(H1);
        board.bitboards[WHITE_BISHOP] = square!(B1) | square!(G1);
        board.bitboards[WHITE_KNIGHT] = square!(C1) | square!(F1);
        board.bitboards[WHITE_PAWN] = RANK_2;
        board.bitboards[BLACK_KING] = square!(E8);
        board.bitboards[BLACK_QUEEN] = square!(D8);
        board.bitboards[BLACK_ROOK] = square!(A8) | square!(H8);
        board.bitboards[BLACK_BISHOP] = square!(B8) | square!(G8);
        board.bitboards[BLACK_KNIGHT] = square!(C8) | square!(F8);
        board.bitboards[BLACK_PAWN] = RANK_7;

        board.castling = Castling::initial();

        board
    }
}
