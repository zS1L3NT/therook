use super::*;

impl Board {
    pub fn calculate_moves(&self) -> Vec<Move> {
        timed!("calculated moves", {
            let mut moves: Vec<Move> = vec![];

            let color = self.turn;
            let opponent = self.turn.opposite();

            let friendlies = self.colors[color];
            let enemies = self.colors[opponent];
            let occupancy = friendlies | enemies;

            // Return early because double checks means only King can move
            if self.check_state[color] == CheckState::Double {
                let tile = Tile::from(self.pieces[color | PieceType::King]);

                let mut attacks =
                    self.computed
                        .attack_masks
                        .get(color, PieceType::King, tile, occupancy);

                // Don't attack friendly pieces
                attacks &= !friendlies;

                // Don't allow King to move into attacked squares
                attacks &= !self.attacks[opponent];

                for _index in attacks {
                    moves.push(Move::new(tile, Tile::from(_index), MoveFlag::None));
                }

                return moves;
            }

            for index in friendlies {
                let tile = Tile::from(index);
                let bitboard = Bitboard::from(tile);
                let piece = self.squares[index as usize].unwrap();

                let color = piece.get_color();
                let r#type = piece.get_type();

                let mut attacks = self
                    .computed
                    .attack_masks
                    .get(color, r#type, tile, occupancy);

                // Don't attack friendly pieces
                attacks &= !friendlies;

                // Don't allow King to move into attacked squares
                if r#type == PieceType::King {
                    attacks &= !self.attacks[opponent];

                    if self.castling.can((color | PieceType::King).into()) {
                        let right_1 = tile << 1;
                        let right_2 = tile << 2;

                        if self.check_state[color] == CheckState::None
                            && self.squares[right_1].is_none()
                            && self.squares[right_2].is_none()
                            && (self.attacks[opponent] & right_1).is_none()
                            && (self.attacks[opponent] & right_2).is_none()
                        {
                            attacks |= Bitboard::from(right_2);
                        }
                    }

                    if self.castling.can((color | PieceType::Queen).into()) {
                        let left_1 = tile >> 1;
                        let left_2 = tile >> 2;
                        let left_3 = tile >> 3;

                        if self.check_state[color] == CheckState::None
                            && self.squares[left_1].is_none()
                            && self.squares[left_2].is_none()
                            && self.squares[left_3].is_none()
                            && (self.attacks[opponent] & left_1).is_none()
                            && (self.attacks[opponent] & left_2).is_none()
                        {
                            attacks |= Bitboard::from(left_2);
                        }
                    }
                }

                if r#type == PieceType::Pawn {
                    let mut can_enpassant = false;

                    // Check for enpassant before removing empty attack squares, since enpassant technically attacks an empty square
                    if (attacks & self.enpassant).is_some() {
                        let diagonal_pinned = (self.pin_lines[color] & self.enpassant).is_some();
                        let orthogonal_pinned = {
                            // Annoyingly long algorithm to calculate orthogonal pins
                            let capturing_pawn = tile;
                            let captured_pawn = Tile::from(
                                (index & 56) + (u8::from(Tile::from(self.enpassant)) & 7),
                            );

                            let enemy_orthogonals = self.pieces[opponent | PieceType::Rook]
                                | self.pieces[opponent | PieceType::Queen];

                            let rook_attacks_from_king_without_pawns =
                                self.computed.attack_masks.get(
                                    color,
                                    PieceType::Rook,
                                    Tile::from(self.pieces[color | PieceType::King]),
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
                        attacks |= self.enpassant;
                    }

                    if color == PieceColor::White && self.squares[tile << 8].is_none() {
                        attacks |= bitboard << 8u64;

                        if index >> 3 == 1 && self.squares[tile << 16].is_none() {
                            attacks |= bitboard << 16u64;
                        }
                    }

                    if color == PieceColor::Black && self.squares[tile >> 8].is_none() {
                        attacks |= bitboard >> 8u64;

                        if index >> 3 == 6 && self.squares[tile >> 16].is_none() {
                            attacks |= bitboard >> 16u64;
                        }
                    }
                }

                // If in check and the piece is not a king
                if r#type != PieceType::King {
                    if let CheckState::Single(attacker) = self.check_state[color] {
                        // Try to resolve the check by capturing or blocking
                        attacks &= attacker
                            | self
                                .computed
                                .obstruction_masks
                                .get(attacker, Tile::from(self.pieces[color | PieceType::King]));
                    }
                }

                // If piece is pinned, only allow moves that keep piece within pin line
                if (self.pin_lines[color] & tile).is_some() {
                    attacks &= self.pin_lines[color];
                }

                for _index in attacks {
                    let _tile = Tile::from(_index);
                    let _rank = _index >> 3;
                    let _file = _index & 7;

                    let mut flag = MoveFlag::None;

                    if r#type == PieceType::Pawn {
                        if index.abs_diff(_index) == 16 {
                            flag = MoveFlag::PawnDash;
                        }

                        if self.enpassant.is_some()
                            && _file == (u8::from(Tile::from(self.enpassant)) & 7)
                        {
                            flag = MoveFlag::EnPassant;
                        }

                        if _rank == 0 || _rank == 7 {
                            moves.push(Move::new(tile, _tile, MoveFlag::PromoteQueen));
                            moves.push(Move::new(tile, _tile, MoveFlag::PromoteRook));
                            moves.push(Move::new(tile, _tile, MoveFlag::PromoteBishop));
                            moves.push(Move::new(tile, _tile, MoveFlag::PromoteKnight));
                            continue;
                        }
                    }

                    if r#type == PieceType::King && index.abs_diff(_index) == 2 {
                        flag = MoveFlag::Castle;
                    }

                    moves.push(Move::new(tile, _tile, flag));
                }
            }

            moves
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod double_check {
        use super::*;

        #[test]
        fn double_check() {
            let mut board = Board::fen("4k3/8/8/8/1b6/4r3/8/4K3 w - - 0 1".into()).unwrap();
            board.check_state[PieceColor::White] = CheckState::Double;

            assert_eq!(
                board.calculate_moves(),
                vec![
                    Move::new(tile!(E1), tile!(D1), MoveFlag::None),
                    Move::new(tile!(E1), tile!(F1), MoveFlag::None),
                    Move::new(tile!(E1), tile!(F2), MoveFlag::None),
                ]
            );
        }

        #[test]
        fn double_check_forced() {
            let mut board = Board::fen("4k3/8/1b6/4r3/8/4K3/r7/4n3 w - - 0 1".into()).unwrap();
            board.check_state[PieceColor::White] = CheckState::Double;

            assert_eq!(
                board.calculate_moves(),
                vec![Move::new(tile!(E3), tile!(F4), MoveFlag::None)]
            );
        }
    }

    mod en_passant {
        use super::*;

        #[test]
        fn allowed() {
            let board = Board::fen("4k3/8/8/3pP3/4K3/8/8/8 w - d6 0 1".into()).unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_flag() == MoveFlag::EnPassant),
                Some(&Move::new(tile!(E5), tile!(D6), MoveFlag::EnPassant))
            );
        }

        #[test]
        fn disallowed_when_orthogonal_pinned() {
            let board = Board::fen("4k3/8/8/2rpPK2/8/8/8/8 w - d6 0 1".into()).unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_start() == tile!(E5)
                    && m.get_end() == tile!(D6)
                    && m.get_flag() == MoveFlag::EnPassant),
                None
            );
        }

        #[test]
        fn disallowed_when_diagonal_pinned() {
            let board = Board::fen("4k3/8/2b5/3pP3/4K3/8/8/8 w - - 0 1".into()).unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_start() == tile!(E5)
                    && m.get_end() == tile!(D6)
                    && m.get_flag() == MoveFlag::EnPassant),
                None
            );
        }

        #[test]
        fn disallowed_when_both_pinned() {
            let board = Board::fen("8/6bb/8/8/R1pP2k1/4P3/P7/K7 b - d3 0 1".into()).unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().find(|m| m.get_start() == tile!(E5)
                    && m.get_end() == tile!(D6)
                    && m.get_flag() == MoveFlag::EnPassant),
                None
            );
        }
    }

    mod castling {
        use super::*;

        #[test]
        fn allowed() {
            let board = Board::fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1".into()).unwrap();
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
            let board = Board::fen("4k3/8/8/8/4b3/8/8/R3K2R w KQ - 0 1".into()).unwrap();
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
            let mut board = Board::fen("4k3/8/8/8/8/2b5/8/R3K2R w KQ - 0 1".into()).unwrap();
            board.check_state[PieceColor::White] = CheckState::Single(tile!(C3));

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
            let board = Board::fen("4k3/8/8/8/8/4b3/8/R3K2R w KQ - 0 1".into()).unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves
                    .iter()
                    .filter(|m| m.get_flag() == MoveFlag::Castle)
                    .count(),
                0
            );

            let board = Board::fen("4k3/8/8/8/8/8/4b3/R3K2R w KQ - 0 1".into()).unwrap();
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
            let board = Board::fen("4k3/P7/8/8/8/8/8/4K3 w - - 0 1".into()).unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().filter(|m| m.get_start() == tile!(A7)).count(),
                4
            );
        }

        #[test]
        fn black() {
            let board = Board::fen("4k3/8/8/8/8/8/p7/4K3 b - - 0 1".into()).unwrap();
            let moves = board.calculate_moves();

            assert_eq!(
                moves.iter().filter(|m| m.get_start() == tile!(A2)).count(),
                4
            );
        }
    }
}
