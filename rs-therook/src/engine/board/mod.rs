mod _debug;
mod _moves;
mod bitboard;
mod castling;

pub use bitboard::*;
pub use castling::*;

use super::*;
use therook::square;

pub struct Board {
    pub bitboards: [Bitboard; 12],
    pub turn: Team,
    pub castling: Castling,
    pub enpassant: Bitboard,
    pub halfmove: u8,
    pub fullmove: u8,
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
        }
    }

    pub fn initial() -> Self {
        let mut board = Board::new();

        board.bitboards[WhiteKing] = square!(E1);
        board.bitboards[WhiteQueen] = square!(D1);
        board.bitboards[WhiteRook] = square!(A1) | square!(H1);
        board.bitboards[WhiteBishop] = square!(B1) | square!(G1);
        board.bitboards[WhiteKnight] = square!(C1) | square!(F1);
        board.bitboards[WhitePawn] = RANK_2;
        board.bitboards[BlackKing] = square!(E8);
        board.bitboards[BlackQueen] = square!(D8);
        board.bitboards[BlackRook] = square!(A8) | square!(H8);
        board.bitboards[BlackBishop] = square!(B8) | square!(G8);
        board.bitboards[BlackKnight] = square!(C8) | square!(F8);
        board.bitboards[BlackPawn] = RANK_7;

        board.castling = Castling::initial();

        board
    }
}
