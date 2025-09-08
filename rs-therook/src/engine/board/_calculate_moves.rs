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

            let pin_lines = self.pin_lines[color];
            let check_count = self.check_count[color];

            // Return early because double checks means only King can move
            if check_count == 2 {
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

                        if check_count == 0
                            && self.squares[right_1].is_none()
                            && self.squares[right_2].is_none()
                            && (self.attacks[opponent] & right_1).is_empty()
                            && (self.attacks[opponent] & right_2).is_empty()
                        {
                            attacks |= Bitboard::from(right_2);
                        }
                    }

                    if self.castling.can((color | PieceType::Queen).into()) {
                        let left_1 = tile >> 1;
                        let left_2 = tile >> 2;
                        let left_3 = tile >> 3;

                        if check_count == 0
                            && self.squares[left_1].is_none()
                            && self.squares[left_2].is_none()
                            && self.squares[left_3].is_none()
                            && (self.attacks[opponent] & left_1).is_empty()
                            && (self.attacks[opponent] & left_2).is_empty()
                        {
                            attacks |= Bitboard::from(left_2);
                        }
                    }
                }

                if r#type == PieceType::Pawn {
                    // Only attack when there is an enemy piece
                    attacks &= enemies;

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

                if !(pin_lines & tile).is_empty() {
                    // Piece is pinned, only allow moves that keep piece within pin line
                    attacks &= pin_lines;
                }

                for _index in attacks {
                    let _tile = Tile::from(_index);

                    if r#type == PieceType::Pawn && index.abs_diff(_index) == 16 {
                        moves.push(Move::new(tile, _tile, MoveFlag::PawnDash));
                    } else if r#type == PieceType::King && index.abs_diff(_index) == 2 {
                        moves.push(Move::new(tile, _tile, MoveFlag::Castle));
                    } else {
                        moves.push(Move::new(tile, _tile, MoveFlag::None));
                    }
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
            board.check_count[PieceColor::White] = 2;

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
            board.check_count[PieceColor::White] = 2;

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
            board.check_count[PieceColor::White] = 1;
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
}
