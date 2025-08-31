mod _debug;
mod _index;
mod _make_move;
mod _moves;
mod _undo_move;
mod castling;

use super::*;
use castling::*;
use therook::{bitboard, tile};

pub struct Board {
    // From FEN
    pub bitboards: [Bitboard; 12],
    pub turn: PieceColor,
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
            turn: PieceColor::White,
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
        board.squares[tile!(E1)] = Some(WHITE_KING);

        board.bitboards[WHITE_QUEEN] = bitboard!(D1);
        board.squares[tile!(D1)] = Some(WHITE_QUEEN);

        board.bitboards[WHITE_ROOK] = bitboard!(A1) | bitboard!(H1);
        board.squares[tile!(A1)] = Some(WHITE_ROOK);
        board.squares[tile!(H1)] = Some(WHITE_ROOK);

        board.bitboards[WHITE_BISHOP] = bitboard!(B1) | bitboard!(G1);
        board.squares[tile!(B1)] = Some(WHITE_BISHOP);
        board.squares[tile!(G1)] = Some(WHITE_BISHOP);

        board.bitboards[WHITE_KNIGHT] = bitboard!(C1) | bitboard!(F1);
        board.squares[tile!(C1)] = Some(WHITE_KNIGHT);
        board.squares[tile!(F1)] = Some(WHITE_KNIGHT);

        board.bitboards[WHITE_PAWN] = RANK_2;
        for tile in RANK_2.get_tiles() {
            board.squares[tile] = Some(WHITE_PAWN);
        }

        board.bitboards[BLACK_KING] = bitboard!(E8);
        board.squares[tile!(E8)] = Some(BLACK_KING);

        board.bitboards[BLACK_QUEEN] = bitboard!(D8);
        board.squares[tile!(D8)] = Some(BLACK_QUEEN);

        board.bitboards[BLACK_ROOK] = bitboard!(A8) | bitboard!(H8);
        board.squares[tile!(A8)] = Some(BLACK_ROOK);
        board.squares[tile!(H8)] = Some(BLACK_ROOK);

        board.bitboards[BLACK_BISHOP] = bitboard!(B8) | bitboard!(G8);
        board.squares[tile!(B8)] = Some(BLACK_BISHOP);
        board.squares[tile!(G8)] = Some(BLACK_BISHOP);

        board.bitboards[BLACK_KNIGHT] = bitboard!(C8) | bitboard!(F8);
        board.squares[tile!(C8)] = Some(BLACK_KNIGHT);
        board.squares[tile!(F8)] = Some(BLACK_KNIGHT);

        board.bitboards[BLACK_PAWN] = RANK_7;
        for tile in RANK_7.get_tiles() {
            board.squares[tile] = Some(BLACK_PAWN);
        }

        board.castling = Castling::initial();

        board
    }
}
