use super::*;

impl Board {
    pub fn make_move(&mut self, r#move: Move) {
        let start_square = r#move.get_start();
        let end_square = r#move.get_end();
        let flag = r#move.get_flag();
        let promotion_piece_type = r#move.get_promote_piece_type();

        let piece = self.squares[start_square as usize].unwrap();
        let piece_type = piece.get_type();

        let color = piece.get_color();
        let enemy = color.opposite();

        let mut state = self.get_state().clone();

        state.captured = if flag == MoveFlag::EnPassant {
            Some(color.opposite() | PieceType::Pawn)
        } else {
            self.squares[end_square as usize]
        };
        let captured_type = state.captured.map(|p| p.get_type());

        self.clear_square(start_square, piece);

        // Remove the captured tile, the piece on the end square or enpassant square
        if let Some(captured) = state.captured {
            self.clear_square(
                if flag == MoveFlag::EnPassant {
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
        if flag == MoveFlag::PawnDash {
            if color == PieceColor::White {
                state.enpassant = Bitboard::from((start_square & 56) + (end_square & 7) + 8)
            } else {
                state.enpassant = Bitboard::from((start_square & 56) + (end_square & 7) - 8)
            }
        } else if state.enpassant.is_some() {
            state.enpassant = Bitboard::new()
        }

        // Castling rights & move castled rook
        if flag == MoveFlag::Castle {
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
                    match start_square & 7 {
                        0 => state.castling[color | PieceType::Queen] = false,
                        7 => state.castling[color | PieceType::King] = false,
                        _ => unreachable!(),
                    }
                }
            }

            // Check if opponent can still castle
            if state.castling[enemy | PieceType::King] || state.castling[enemy | PieceType::Queen] {
                if captured_type == Some(PieceType::Rook) {
                    match end_square & 7 {
                        0 => state.castling[enemy | PieceType::Queen] = false,
                        7 => state.castling[enemy | PieceType::King] = false,
                        _ => unreachable!(),
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
            let mut board =
                Board::try_from("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 1")
                    .unwrap();

            board.make_move(Move::new(square!(E5), square!(D6), MoveFlag::EnPassant));

            assert_eq!(board.squares[square!(D5)], None);
            assert_eq!(board.squares[square!(E5)], None);
            assert_eq!(board.squares[square!(D6)], Some(WHITE_PAWN));
            assert_eq!(
                &board,
                "rnbqkbnr/ppp1pppp/3P4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
            )
        }

        #[test]
        fn pawn_dash_sets_enpassant_square() {
            let mut board = Board::initial();

            board.make_move(Move::new(square!(E2), square!(E4), MoveFlag::PawnDash));
            assert_eq!(board.get_state().enpassant, bitboard!(E3));
            assert_eq!(
                &board,
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
            );

            board.make_move(Move::new(square!(D7), square!(D5), MoveFlag::PawnDash));
            assert_eq!(board.get_state().enpassant, bitboard!(D6));
            assert_eq!(
                &board,
                "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"
            );
        }

        #[test]
        fn move_after_enpassant_resets_enpassant_square() {
            let mut board = Board::initial();

            board.make_move(Move::new(square!(E2), square!(E4), MoveFlag::PawnDash));
            assert_eq!(board.get_state().enpassant, bitboard!(E3));
            assert_eq!(
                &board,
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
            );

            board.make_move(Move::new(square!(D7), square!(D6), MoveFlag::None));
            assert_eq!(board.get_state().enpassant, Bitboard::new());
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
            let mut board =
                Board::try_from("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").unwrap();
            let mut castling = [true; 4];

            board.make_move(Move::new(square!(E1), square!(G1), MoveFlag::Castle));

            assert_eq!(board.squares[square!(E1)], None);
            assert_eq!(board.squares[square!(F1)], Some(WHITE_ROOK));
            assert_eq!(board.squares[square!(G1)], Some(WHITE_KING));
            assert_eq!(board.squares[square!(H1)], None);

            castling[WHITE_KING] = false;
            castling[WHITE_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 b kq - 1 1");

            board.make_move(Move::new(square!(E8), square!(G8), MoveFlag::Castle));

            assert_eq!(board.squares[square!(E8)], None);
            assert_eq!(board.squares[square!(F8)], Some(BLACK_ROOK));
            assert_eq!(board.squares[square!(G8)], Some(BLACK_KING));
            assert_eq!(board.squares[square!(H8)], None);

            castling[BLACK_KING] = false;
            castling[BLACK_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 w - - 2 2");
        }

        #[test]
        fn queenside() {
            let mut board =
                Board::try_from("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").unwrap();
            let mut castling = [true; 4];

            board.make_move(Move::new(square!(E1), square!(C1), MoveFlag::Castle));

            assert_eq!(board.squares[square!(A1)], None);
            assert_eq!(board.squares[square!(B1)], None);
            assert_eq!(board.squares[square!(C1)], Some(WHITE_KING));
            assert_eq!(board.squares[square!(D1)], Some(WHITE_ROOK));
            assert_eq!(board.squares[square!(E1)], None);

            castling[WHITE_KING] = false;
            castling[WHITE_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R b kq - 1 1");

            board.make_move(Move::new(square!(E8), square!(C8), MoveFlag::Castle));

            assert_eq!(board.squares[square!(A8)], None);
            assert_eq!(board.squares[square!(B8)], None);
            assert_eq!(board.squares[square!(C8)], Some(BLACK_KING));
            assert_eq!(board.squares[square!(D8)], Some(BLACK_ROOK));
            assert_eq!(board.squares[square!(E8)], None);

            castling[BLACK_KING] = false;
            castling[BLACK_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R w - - 2 2");
        }

        #[test]
        fn moving_rook_loses_rights() {
            let mut board =
                Board::try_from("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").unwrap();
            let mut castling = [true; 4];

            board.make_move(Move::new(square!(H1), square!(G1), MoveFlag::None));

            castling[WHITE_KING] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K1R1 b Qkq - 1 1");

            board.make_move(Move::new(square!(H8), square!(G8), MoveFlag::None));

            castling[BLACK_KING] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k1r1/pppppppp/8/8/8/8/PPPPPPPP/R3K1R1 w Qq - 2 2");

            board.make_move(Move::new(square!(A1), square!(B1), MoveFlag::None));

            castling[WHITE_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k1r1/pppppppp/8/8/8/8/PPPPPPPP/1R2K1R1 b q - 3 2");

            board.make_move(Move::new(square!(A8), square!(B8), MoveFlag::None));

            castling[BLACK_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(
                &board,
                "1r2k1r1/pppppppp/8/8/8/8/PPPPPPPP/1R2K1R1 w - - 4 3"
            );
        }

        #[test]
        fn moving_king_loses_rights() {
            let mut board =
                Board::try_from("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").unwrap();
            let mut castling = [true; 4];

            board.make_move(Move::new(square!(E1), square!(D1), MoveFlag::None));

            castling[WHITE_KING] = false;
            castling[WHITE_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R2K3R b kq - 1 1");

            board.make_move(Move::new(square!(E8), square!(D8), MoveFlag::None));

            castling[BLACK_KING] = false;
            castling[BLACK_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r2k3r/pppppppp/8/8/8/8/PPPPPPPP/R2K3R w - - 2 2");
        }

        #[test]
        fn capturing_rook_revokes_rights() {
            let mut board = Board::try_from("r3k2r/8/8/3BB3/3bb3/8/8/R3K2R w KQkq - 0 1").unwrap();
            let mut castling = [true; 4];

            board.make_move(Move::new(square!(E5), square!(H8), MoveFlag::None));

            castling[BLACK_KING] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k2B/8/8/3B4/3bb3/8/8/R3K2R b KQq - 0 1");

            board.make_move(Move::new(square!(E4), square!(H1), MoveFlag::None));

            castling[WHITE_KING] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "r3k2B/8/8/3B4/3b4/8/8/R3K2b w Qq - 0 2");

            board.make_move(Move::new(square!(D5), square!(A8), MoveFlag::None));

            castling[BLACK_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "B3k2B/8/8/8/3b4/8/8/R3K2b b Q - 0 2");

            board.make_move(Move::new(square!(D4), square!(A1), MoveFlag::None));

            castling[WHITE_QUEEN] = false;
            assert_eq!(board.get_state().castling, castling);
            assert_eq!(&board, "B3k2B/8/8/8/8/8/8/b3K2b w - - 0 3");
        }
    }

    mod promote {
        use super::*;

        #[test]
        fn queen() {
            let mut board = Board::try_from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteQueen));

            assert_eq!(board.squares[square!(A7)], None);
            assert_eq!(board.squares[square!(A8)], Some(WHITE_QUEEN));
            assert_eq!(&board, "Q3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn rook() {
            let mut board = Board::try_from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteRook));

            assert_eq!(board.squares[square!(A7)], None);
            assert_eq!(board.squares[square!(A8)], Some(WHITE_ROOK));
            assert_eq!(&board, "R3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn bishop() {
            let mut board = Board::try_from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteBishop));

            assert_eq!(board.squares[square!(A7)], None);
            assert_eq!(board.squares[square!(A8)], Some(WHITE_BISHOP));
            assert_eq!(&board, "B3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn knight() {
            let mut board = Board::try_from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();

            board.make_move(Move::new(square!(A7), square!(A8), MoveFlag::PromoteKnight));

            assert_eq!(board.squares[square!(A7)], None);
            assert_eq!(board.squares[square!(A8)], Some(WHITE_KNIGHT));
            assert_eq!(&board, "N3k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }

        #[test]
        fn queen_with_capture() {
            let mut board = Board::try_from("1n2k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();

            board.make_move(Move::new(square!(A7), square!(B8), MoveFlag::PromoteQueen));

            assert_eq!(board.squares[square!(A7)], None);
            assert_eq!(board.squares[square!(B8)], Some(WHITE_QUEEN));
            assert_eq!(&board, "1Q2k3/8/8/8/8/8/8/4K3 b - - 0 1");
        }
    }
}
