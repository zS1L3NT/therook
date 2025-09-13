use super::*;

impl Board<'_> {
    pub fn undo_move(&mut self, r#move: Move) {
        let start_square = r#move.get_start();
        let end_square = r#move.get_end();

        let flag = r#move.get_flag();
        let is_enpassant = flag == MoveFlag::EnPassant;
        let is_castle = flag == MoveFlag::Castle;
        let is_promote = r#move.get_promote_piece_type().is_some();

        let piece = self.squares[end_square as usize].unwrap();
        let color = piece.get_color();

        let state = self
            .states
            .pop()
            .unwrap_or_else(|| panic!("No board state..."));

        if is_castle {
            let (from_square, to_square, piece) = match end_square {
                square!(G1) => (square!(F1), square!(H1), WHITE_ROOK),
                square!(C1) => (square!(D1), square!(A1), WHITE_ROOK),
                square!(G8) => (square!(F8), square!(H8), BLACK_ROOK),
                square!(C8) => (square!(D8), square!(A8), BLACK_ROOK),
                _ => unreachable!(),
            };

            self.clear_square(from_square, piece);
            self.set_square(to_square, piece);
        }

        if is_promote {
            self.set_square(start_square, color | PieceType::Pawn);
        } else {
            self.set_square(start_square, piece);
        }

        self.clear_square(end_square, piece);

        if let Some(captured) = state.captured {
            self.set_square(
                if is_enpassant {
                    (start_square & 56) + (end_square & 7)
                } else {
                    end_square
                },
                captured,
            );
        }

        // Update the extra state of the board
        if piece.is_slider() {
            self.update_rays(color);
        }

        for color in [PieceColor::White, PieceColor::Black] {
            self.update_attacks(color);
            self.update_pin_lines(color);
        }

        self.turn = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod enpassant {
        use super::*;

        #[test]
        fn resets() {
            let fen = "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let r#move = Move::new(square!(E5), square!(D6), MoveFlag::EnPassant);

            board.make_move(r#move);

            board.undo_move(r#move);

            assert_eq!(&board, fen);
        }
    }

    mod castle {
        use super::*;

        #[test]
        fn kingside() {
            let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let white_move = Move::new(square!(E1), square!(G1), MoveFlag::Castle);
            let black_move = Move::new(square!(E8), square!(G8), MoveFlag::Castle);

            board.make_move(white_move);
            board.make_move(black_move);

            board.undo_move(black_move);
            board.undo_move(white_move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn queenside() {
            let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let white_move = Move::new(square!(E1), square!(C1), MoveFlag::Castle);
            let black_move = Move::new(square!(E8), square!(C8), MoveFlag::Castle);

            board.make_move(white_move);
            board.make_move(black_move);

            board.undo_move(black_move);
            board.undo_move(white_move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn reset_castling_rights_when_moving_rooks() {
            let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let h1_move = Move::new(square!(H1), square!(G1), MoveFlag::None);
            let h8_move = Move::new(square!(H8), square!(G8), MoveFlag::None);
            let a1_move = Move::new(square!(A1), square!(B1), MoveFlag::None);
            let a8_move = Move::new(square!(A8), square!(B8), MoveFlag::None);

            board.make_move(h1_move);
            board.make_move(h8_move);
            board.make_move(a1_move);
            board.make_move(a8_move);

            board.undo_move(a8_move);
            board.undo_move(a1_move);
            board.undo_move(h8_move);
            board.undo_move(h1_move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn reset_castling_rights_when_moving_king() {
            let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let white_move = Move::new(square!(E1), square!(D1), MoveFlag::None);
            let black_move = Move::new(square!(E8), square!(D8), MoveFlag::None);

            board.make_move(white_move);
            board.make_move(black_move);

            board.undo_move(black_move);
            board.undo_move(white_move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn reset_castling_rights_after_rooks_captured() {
            let fen = "r3k2r/8/8/3BB3/3bb3/8/8/R3K2R w KQkq - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let h8_capture = Move::new(square!(E5), square!(H8), MoveFlag::None);
            let h1_capture = Move::new(square!(E4), square!(H1), MoveFlag::None);
            let a8_capture = Move::new(square!(D5), square!(A8), MoveFlag::None);
            let a1_capture = Move::new(square!(D4), square!(A1), MoveFlag::None);

            board.make_move(h8_capture);
            board.make_move(h1_capture);
            board.make_move(a8_capture);
            board.make_move(a1_capture);

            board.undo_move(a1_capture);
            board.undo_move(a8_capture);
            board.undo_move(h1_capture);
            board.undo_move(h8_capture);

            assert_eq!(&board, fen);
        }
    }

    mod promote {
        use super::*;

        #[test]
        fn queen() {
            let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let r#move = Move::new(square!(A7), square!(A8), MoveFlag::PromoteQueen);

            board.make_move(r#move);

            board.undo_move(r#move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn rook() {
            let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let r#move = Move::new(square!(A7), square!(A8), MoveFlag::PromoteRook);

            board.make_move(r#move);

            board.undo_move(r#move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn bishop() {
            let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let r#move = Move::new(square!(A7), square!(A8), MoveFlag::PromoteBishop);

            board.make_move(r#move);

            board.undo_move(r#move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn knight() {
            let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let r#move = Move::new(square!(A7), square!(A8), MoveFlag::PromoteKnight);

            board.make_move(r#move);

            board.undo_move(r#move);

            assert_eq!(&board, fen);
        }

        #[test]
        fn queen_with_capture() {
            let fen = "1n2k3/P7/8/8/8/8/8/4K3 w - - 0 1";
            let computed = Computed::new();
            let mut board = Board::from_fen(fen, &computed);
            let r#move = Move::new(square!(A7), square!(B8), MoveFlag::PromoteQueen);

            board.make_move(r#move);

            board.undo_move(r#move);

            assert_eq!(&board, fen);
        }
    }
}
