use super::*;

impl Board {
    pub fn calculate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        let color = self.turn;
        let enemy = self.turn.opposite();

        let friendlies = self.colors[color];
        let enemies = self.colors[enemy];
        let occupancy = friendlies | enemies;
        let state = self.get_state();

        let king_square = u8::try_from(self.pieces[color | PieceType::King]).unwrap();

        // Return early because double checks means only King can move
        if self.check_state[color] == CheckState::Double {
            let mut attacks =
                self.computed
                    .attacks
                    .get(color, PieceType::King, king_square, occupancy);

            // Don't attack friendly pieces
            attacks &= !friendlies;

            // Don't allow King to move into attacked squares
            attacks &= !self.attacks[enemy];

            for _square in attacks {
                moves.push(Move::new(king_square, _square, MoveFlag::None));
            }

            return moves;
        }

        for square in friendlies {
            let bitboard = Bitboard::from(square);
            let piece = self.squares[square as usize].unwrap();

            let color = piece.get_color();
            let r#type = piece.get_type();

            let mut attacks = self.computed.attacks.get(color, r#type, square, occupancy);
            let mut can_enpassant = false;

            // Don't attack friendly pieces
            attacks &= !friendlies;

            // Don't allow King to move into attacked squares
            if r#type == PieceType::King {
                attacks &= !self.attacks[enemy];

                if state.castling[color | PieceType::King]
                    && self.check_state[color] == CheckState::None
                    && self.squares[square as usize + 1].is_none()
                    && self.squares[square as usize + 2].is_none()
                    && (self.attacks[enemy] & Bitboard::from(square + 1)).is_none()
                    && (self.attacks[enemy] & Bitboard::from(square + 2)).is_none()
                {
                    attacks |= Bitboard::from(square + 2);
                }

                if state.castling[color | PieceType::Queen]
                    && self.check_state[color] == CheckState::None
                    && self.squares[square as usize - 1].is_none()
                    && self.squares[square as usize - 2].is_none()
                    && self.squares[square as usize - 3].is_none()
                    && (self.attacks[enemy] & Bitboard::from(square - 1)).is_none()
                    && (self.attacks[enemy] & Bitboard::from(square - 2)).is_none()
                {
                    attacks |= Bitboard::from(square - 2);
                }
            }

            if r#type == PieceType::Pawn {
                // Check for enpassant before removing empty attack squares, since enpassant technically attacks an empty square
                if (attacks & state.enpassant).is_some() {
                    let enpassant_square = u8::try_from(state.enpassant).unwrap();

                    let diagonal_pinned = (self.pin_lines[color] & state.enpassant).is_some();
                    let orthogonal_pinned = {
                        // Annoyingly long algorithm to calculate orthogonal pins
                        let capturing_pawn = square;
                        let captured_pawn = (square & 56) + (enpassant_square & 7);

                        let enemy_orthogonals = self.pieces[enemy | PieceType::Rook]
                            | self.pieces[enemy | PieceType::Queen];

                        let rook_attacks_from_king_without_pawns = self.computed.attacks.get(
                            color,
                            PieceType::Rook,
                            king_square,
                            occupancy ^ capturing_pawn ^ captured_pawn,
                        );

                        (rook_attacks_from_king_without_pawns & enemy_orthogonals).is_some()
                    };

                    can_enpassant = !diagonal_pinned && !orthogonal_pinned;
                }

                // Only attack when there is an enemy piece
                attacks &= enemies;

                // With the one exception of enpassant
                if can_enpassant {
                    attacks |= state.enpassant;
                }

                if color == PieceColor::White && self.squares[square as usize + 8].is_none() {
                    attacks |= bitboard << 8;

                    if square >> 3 == 1 && self.squares[square as usize + 16].is_none() {
                        attacks |= bitboard << 16;
                    }
                }

                if color == PieceColor::Black && self.squares[square as usize - 8].is_none() {
                    attacks |= bitboard >> 8;

                    if square >> 3 == 6 && self.squares[square as usize - 16].is_none() {
                        attacks |= bitboard >> 16;
                    }
                }
            }

            // If in check and the piece is not a king
            if r#type != PieceType::King {
                if let CheckState::Single(attacker) = self.check_state[color] {
                    // Try to resolve the check by blocking the attack
                    let mut resolving = self.computed.betweens.get(attacker, king_square);

                    // Or capturing the attacker
                    resolving |= attacker;

                    // If can enpassant and the attacker is the enpassant target
                    if can_enpassant {
                        let enpassant_square = u8::try_from(state.enpassant).unwrap();

                        // Rank of the current piece and file of the enpassant square gives the piece to be captured
                        let capture_square = (square & 56) + (enpassant_square & 7);

                        // Set the resolving square to the enpassant square instead of the capture square
                        if attacker == capture_square {
                            resolving ^= capture_square;
                            resolving |= enpassant_square;
                        }
                    }

                    attacks &= resolving;
                }
            }

            // If piece is pinned, only allow moves that keep piece within pin line
            if (self.pin_lines[color] & Bitboard::from(square)).is_some() {
                attacks &= self.pin_lines[color];
            }

            for _square in attacks {
                let _rank = _square >> 3;
                let _file = _square & 7;

                let mut flag = MoveFlag::None;

                if r#type == PieceType::Pawn {
                    if square.abs_diff(_square) == 16 {
                        flag = MoveFlag::PawnDash;
                    }

                    if state.enpassant.is_some()
                        && _file == (u8::try_from(state.enpassant).unwrap() & 7)
                    {
                        flag = MoveFlag::EnPassant;
                    }

                    if _rank == 0 || _rank == 7 {
                        moves.push(Move::new(square, _square, MoveFlag::PromoteQueen));
                        moves.push(Move::new(square, _square, MoveFlag::PromoteRook));
                        moves.push(Move::new(square, _square, MoveFlag::PromoteBishop));
                        moves.push(Move::new(square, _square, MoveFlag::PromoteKnight));
                        continue;
                    }
                }

                if r#type == PieceType::King && square.abs_diff(_square) == 2 {
                    flag = MoveFlag::Castle;
                }

                moves.push(Move::new(square, _square, flag));
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod double_check {
        use super::*;

        #[test]
        fn double_check() {
            let mut board = Board::try_from("4k3/8/8/8/1b6/4r3/8/4K3 w - - 0 1").unwrap();
            board.check_state[PieceColor::White] = CheckState::Double;

            assert_eq!(
                board.calculate_moves(),
                vec![
                    Move::new(square!(E1), square!(D1), MoveFlag::None),
                    Move::new(square!(E1), square!(F1), MoveFlag::None),
                    Move::new(square!(E1), square!(F2), MoveFlag::None),
                ]
            );
        }

        #[test]
        fn double_check_forced() {
            let mut board = Board::try_from("4k3/8/1b6/4r3/8/4K3/r7/4n3 w - - 0 1").unwrap();
            board.check_state[PieceColor::White] = CheckState::Double;

            assert_eq!(
                board.calculate_moves(),
                vec![Move::new(square!(E3), square!(F4), MoveFlag::None)]
            );
        }
    }

    mod enpassant {
        use super::*;

        #[test]
        fn allowed() {
            let board = Board::try_from("4k3/8/8/3pP3/4K3/8/8/8 w - d6 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_flag() == MoveFlag::EnPassant),
                Some(&Move::new(square!(E5), square!(D6), MoveFlag::EnPassant))
            );
        }

        #[test]
        fn disallowed_when_orthogonal_pinned() {
            let board = Board::try_from("4k3/8/8/2rpPK2/8/8/8/8 w - d6 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_start() == square!(E5)
                    && m.get_end() == square!(D6)
                    && m.get_flag() == MoveFlag::EnPassant),
                None
            );
        }

        #[test]
        fn disallowed_when_diagonal_pinned() {
            let board = Board::try_from("4k3/8/2b5/3pP3/4K3/8/8/8 w - - 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_start() == square!(E5)
                    && m.get_end() == square!(D6)
                    && m.get_flag() == MoveFlag::EnPassant),
                None
            );
        }

        #[test]
        fn disallowed_when_both_pinned() {
            let board = Board::try_from("8/6bb/8/8/R1pP2k1/4P3/P7/K7 b - d3 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_start() == square!(E5)
                    && m.get_end() == square!(D6)
                    && m.get_flag() == MoveFlag::EnPassant),
                None
            );
        }
    }

    mod castle {
        use super::*;

        #[test]
        fn allowed() {
            let board = Board::try_from("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_flag() == MoveFlag::Castle)
                    .count(),
                2
            );
        }

        #[test]
        fn allowed_when_file_b_attacked() {
            let board = Board::try_from("4k3/8/8/8/4b3/8/8/R3K2R w KQ - 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_flag() == MoveFlag::Castle)
                    .count(),
                2
            );
        }

        #[test]
        fn disallowed_when_king_checked() {
            let mut board = Board::try_from("4k3/8/8/8/8/2b5/8/R3K2R w KQ - 0 1").unwrap();
            board.check_state[PieceColor::White] = CheckState::Single(u8::from(square!(C3)));

            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_flag() == MoveFlag::Castle)
                    .count(),
                0
            );
        }

        #[test]
        fn disallowed_when_target_square_checked() {
            let board = Board::try_from("4k3/8/8/8/8/4b3/8/R3K2R w KQ - 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_flag() == MoveFlag::Castle)
                    .count(),
                0
            );

            let board = Board::try_from("4k3/8/8/8/8/8/4b3/R3K2R w KQ - 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_flag() == MoveFlag::Castle)
                    .count(),
                0
            );
        }
    }

    mod promote {
        use super::*;

        #[test]
        fn white() {
            let board = Board::try_from("4k3/P7/8/8/8/8/8/4K3 w - - 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_start() == square!(A7))
                    .count(),
                4
            );
        }

        #[test]
        fn black() {
            let board = Board::try_from("4k3/8/8/8/8/8/p7/4K3 b - - 0 1").unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_start() == square!(A2))
                    .count(),
                4
            );
        }
    }
}
