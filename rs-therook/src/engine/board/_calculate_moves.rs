use super::*;

impl Board {
    pub fn calculate_moves(&self) -> Vec<Move> {
        timed!("calculated moves", {
            let mut moves: Vec<Move> = vec![];
            let friendlies = self.colors[self.turn];
            let enemies = self.colors[self.turn.opposite()];

            for tile in friendlies.get_tiles() {
                let index = u8::from(tile);
                let piece = self.squares[index as usize].unwrap();

                let color = piece.get_color();
                let r#type = piece.get_type();

                let mut attacks =
                    self.computed
                        .attack_masks
                        .get(color, r#type, tile, friendlies | enemies);

                // Don't attack friendly pieces
                attacks &= !friendlies;

                if r#type == PieceType::Pawn {
                    // Only attack when there is an enemy piece
                    attacks &= enemies;

                    if color == PieceColor::White && self.squares[tile << 8].is_none() {
                        moves.push(Move::new(tile, tile << 8, MoveFlag::None));

                        if index >> 3 == 1 && self.squares[tile << 16].is_none() {
                            moves.push(Move::new(tile, tile << 16, MoveFlag::PawnDash));
                        }
                    }

                    if color == PieceColor::Black && self.squares[tile >> 8].is_none() {
                        moves.push(Move::new(tile, tile >> 8, MoveFlag::None));

                        if index >> 3 == 6 && self.squares[tile >> 16].is_none() {
                            moves.push(Move::new(tile, tile >> 16, MoveFlag::PawnDash));
                        }
                    }
                }

                for attack in attacks.get_tiles() {
                    moves.push(Move::new(tile, attack, MoveFlag::None));
                }
            }

            moves
        })
    }
}
