use super::*;

impl Board {
    pub fn calculate_moves(&self) -> Vec<Move> {
        timed!("calculated moves", {
            let mut moves: Vec<Move> = vec![];

            let color = self.turn;
            let opponent = self.turn.opposite();

            let friendlies = self.colors[color];
            let enemies = self.colors[opponent];
            let occupied = friendlies | enemies;

            let pin_lines = self.pin_lines[color];

            for tile in friendlies.get_tiles() {
                let index = u8::from(tile);
                let bitboard = Bitboard::from(tile);
                let piece = self.squares[index as usize].unwrap();

                let color = piece.get_color();
                let r#type = piece.get_type();

                let mut attacks = self
                    .computed
                    .attack_masks
                    .get(color, r#type, tile, occupied);

                // Don't attack friendly pieces
                attacks &= !friendlies;

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

                for attack in attacks.get_tiles() {
                    if r#type == PieceType::Pawn && index.abs_diff(u8::from(attack)) == 16 {
                        moves.push(Move::new(tile, attack, MoveFlag::PawnDash));
                    } else {
                        moves.push(Move::new(tile, attack, MoveFlag::None));
                    }
                }
            }

            moves
        })
    }
}
