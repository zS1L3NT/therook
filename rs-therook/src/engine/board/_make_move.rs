use super::*;

impl Board<'_> {
    pub fn make_move(&mut self, r#move: Move) {
        let start_square = r#move.get_start();
        let end_square = r#move.get_end();

        let flag = r#move.get_flag();
        let is_enpassant = flag == MoveFlag::EnPassant;
        let is_castle = flag == MoveFlag::Castle;
        let is_pawn_dash = flag == MoveFlag::PawnDash;
        let promotion_piece_type = r#move.get_promote_piece_type();

        let piece = self.squares[start_square as usize].unwrap();
        let piece_type = piece.get_type();

        let color = piece.get_color();
        let enemy = color.opposite();

        let mut state = self.get_state().clone();

        state.captured = if is_enpassant {
            Some(color.opposite() | PieceType::Pawn)
        } else {
            self.squares[end_square as usize]
        };
        let captured_type = state.captured.map(|p| p.get_type());

        self.clear_square(start_square, piece);

        // Remove the captured tile, the piece on the end square or enpassant square
        if let Some(captured) = state.captured {
            self.clear_square(
                if is_enpassant {
                    (start_square & 56) + (end_square & 7)
                } else {
                    end_square
                },
                captured,
            );
        }

        // Set piece back on the board
        if let Some(r#type) = promotion_piece_type {
            self.set_square(end_square, color | r#type);
        } else {
            self.set_square(end_square, piece);
        }

        // Update enpassant square
        if is_pawn_dash {
            if color == PieceColor::White {
                state.enpassant = Bitboard::from((start_square & 56) + (end_square & 7) + 8)
            } else {
                state.enpassant = Bitboard::from((start_square & 56) + (end_square & 7) - 8)
            }
        } else if state.enpassant.is_some() {
            state.enpassant = Bitboard::new()
        }

        // Castling rights & move castled rook
        if is_castle {
            let (from_square, to_square, piece) = match end_square {
                square!(G1) => (square!(H1), square!(F1), WHITE_ROOK),
                square!(C1) => (square!(A1), square!(D1), WHITE_ROOK),
                square!(G8) => (square!(H8), square!(F8), BLACK_ROOK),
                square!(C8) => (square!(A8), square!(D8), BLACK_ROOK),
                _ => unreachable!(),
            };

            self.clear_square(from_square, piece);
            self.set_square(to_square, piece);
        }

        if state.castling != [false; 4] {
            // Check if we still can castle
            if state.castling[color | PieceType::King] || state.castling[color | PieceType::Queen] {
                if piece_type == PieceType::King {
                    state.castling[color | PieceType::King] = false;
                    state.castling[color | PieceType::Queen] = false;
                }

                if piece_type == PieceType::Rook {
                    if start_square & 7 == 0 {
                        state.castling[color | PieceType::Queen] = false;
                    }

                    if start_square & 7 == 7 {
                        state.castling[color | PieceType::King] = false;
                    }
                }
            }

            // Check if opponent can still castle
            if state.castling[enemy | PieceType::King] || state.castling[enemy | PieceType::Queen] {
                if captured_type == Some(PieceType::Rook) {
                    if end_square & 7 == 0 {
                        state.castling[enemy | PieceType::Queen] = false;
                    }

                    if end_square & 7 == 7 {
                        state.castling[enemy | PieceType::King] = false;
                    }
                }
            }
        }

        // Increment halfmove & fullmove
        if piece_type != PieceType::Pawn && state.captured.is_none() {
            state.halfmove += 1;
        }

        if color == PieceColor::Black {
            state.fullmove += 1;
        }

        // Update the extra state of the board
        if piece_type.is_slider() {
            self.update_rays(color);
        }

        for color in [PieceColor::White, PieceColor::Black] {
            self.update_attacks(color);
            self.update_pin_lines(color);
        }

        self.states.push(state);

        self.turn = enemy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod enpassant {
        use super::*;

        #[test]
        fn captured_pawn_disappears() {
            let computed = Computed::new();
            let mut board = Board::from_fen(
                "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1",
                &computed,
            );

            board.make_move(Move::new(square!(E5), square!(D6), MoveFlag::EnPassant));
            assert_eq!(
                &board,
                "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
            );
        }

        #[test]
        fn pawn_dash_sets_enpassant_square() {
            let computed = Computed::new();
            let mut board = Board::initial(&computed);

            board.make_move(Move::new(square!(E2), square!(E4), MoveFlag::PawnDash));
            assert_eq!(
                &board,
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
            );

            board.make_move(Move::new(square!(D7), square!(D5), MoveFlag::PawnDash));
            assert_eq!(
                &board,
                "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2"
            );
        }

        #[test]
        fn move_after_enpassant_resets_enpassant_square() {
            let computed = Computed::new();
            let mut board = Board::initial(&computed);

            board.make_move(Move::new(square!(E2), square!(E4), MoveFlag::PawnDash));
            assert_eq!(
                &board,
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
            );

            board.make_move(Move::new(square!(D7), square!(D6), MoveFlag::None));
            assert_eq!(
                &board,
                "rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"
            );
        }
    }

    mod castle {
        use super::*;

        #[test]
        fn kingside() {
            let computed = Computed::new();
            let mut board = Board::from_fen(
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1",
                &computed,
            );

            board.make_move(Move::new(square!(E1), square!(G1), MoveFlag::Castle));
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 b kq - 1 1");

            board.make_move(Move::new(square!(E8), square!(G8), MoveFlag::Castle));
            assert_eq!(&board, "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 w - - 2 2");
        }

        #[test]
        fn queenside() {
            let computed = Computed::new();
            let mut board = Board::from_fen(
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1",
                &computed,
            );

            board.make_move(Move::new(square!(E1), square!(C1), MoveFlag::Castle));
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R b kq - 1 1");

            board.make_move(Move::new(square!(E8), square!(C8), MoveFlag::Castle));
            assert_eq!(&board, "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R w - - 2 2");
        }

        #[test]
        fn moving_rook_loses_rights() {
            let computed = Computed::new();
            let mut board = Board::from_fen(
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1",
                &computed,
            );

            board.make_move(Move::new(square!(H1), square!(G1), MoveFlag::None));
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K1R1 b Qkq - 1 1");

            board.make_move(Move::new(square!(H8), square!(G8), MoveFlag::None));
            assert_eq!(&board, "r3k1r1/pppppppp/8/8/8/8/PPPPPPPP/R3K1R1 w Qq - 2 2");

            board.make_move(Move::new(square!(A1), square!(B1), MoveFlag::None));
            assert_eq!(&board, "r3k1r1/pppppppp/8/8/8/8/PPPPPPPP/1R2K1R1 b q - 3 2");

            board.make_move(Move::new(square!(A8), square!(B8), MoveFlag::None));
            assert_eq!(
                &board,
                "1r2k1r1/pppppppp/8/8/8/8/PPPPPPPP/1R2K1R1 w - - 4 3"
            );
        }

        #[test]
        fn moving_king_loses_rights() {
            let computed = Computed::new();
            let mut board = Board::from_fen(
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1",
                &computed,
            );

            board.make_move(Move::new(square!(E1), square!(D1), MoveFlag::None));
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R2K3R b kq - 1 1");

            board.make_move(Move::new(square!(E8), square!(D8), MoveFlag::None));
            assert_eq!(&board, "r2k3r/pppppppp/8/8/8/8/PPPPPPPP/R2K3R w - - 2 2");
        }

        #[test]
        fn capturing_rook_revokes_rights() {
            let computed = Computed::new();
            let mut board =
                Board::from_fen("r3k2r/8/8/3BB3/3bb3/8/8/R3K2R w KQkq - 0 1", &computed);

            board.make_move(Move::new(square!(E5), square!(H8), MoveFlag::None));
            assert_eq!(&board, "r3k2B/8/8/3B4/3bb3/8/8/R3K2R b KQq - 0 1");

            board.make_move(Move::new(square!(E4), square!(H1), MoveFlag::None));
            assert_eq!(&board, "r3k2B/8/8/3B4/3b4/8/8/R3K2b w Qq - 0 2");

            board.make_move(Move::new(square!(D5), square!(A8), MoveFlag::None));
            assert_eq!(&board, "B3k2B/8/8/8/3b4/8/8/R3K2b b Q - 0 2");

            board.make_move(Move::new(square!(D4), square!(A1), MoveFlag::None));
            assert_eq!(&board, "B3k2B/8/8/8/8/8/8/b3K2b w - - 0 3");
        }
    }

    mod promote {
        use super::*;

        #[test]
        fn queen() {
            let computed = Computed::new();
            let mut board = Board::from_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", &computed);

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteQueen));
            assert_eq!(&board, "Q3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn rook() {
            let computed = Computed::new();
            let mut board = Board::from_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", &computed);

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteRook));
            assert_eq!(&board, "R3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn bishop() {
            let computed = Computed::new();
            let mut board = Board::from_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", &computed);

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteBishop));
            assert_eq!(&board, "B3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn knight() {
            let computed = Computed::new();
            let mut board = Board::from_fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", &computed);

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteKnight));
            assert_eq!(&board, "N3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn queen_with_capture() {
            let computed = Computed::new();
            let mut board = Board::from_fen("1n2k3/P7/8/8/8/8/8/4K3 w - - 0 1", &computed);

            board.make_move(Move::new(square!(A7), square!(B8), MoveFlag::PromoteQueen));
            assert_eq!(&board, "1Q2k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }
    }
}
