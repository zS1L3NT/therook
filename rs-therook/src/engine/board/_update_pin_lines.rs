use super::*;

// https://www.chessprogramming.org/Checks_and_Pinned_Pieces_%28Bitboards%29#Absolute_Pins
impl Board {
    pub fn update_pin_lines(&mut self, color: PieceColor) {
        timed!(format!("updated pin lines for {:?}", char::from(color)), {
            let opponent = color.opposite();

            let friendlies = self.colors[color];
            let enemies = self.colors[opponent];
            let occupied = friendlies | enemies;

            let king = Tile::from(self.pieces[color | PieceType::King]);

            self.pin_lines[color] = Bitboard::new();

            let mut pinner = self
                .computed
                .xray_orthogonal_attacks(occupied, friendlies, king);
            pinner &=
                self.pieces[opponent | PieceType::Queen] | self.pieces[opponent | PieceType::Rook];

            self.clear_pinner(&mut pinner, color, king);

            let mut pinner = self
                .computed
                .xray_diagonal_attacks(occupied, friendlies, king);
            pinner &= self.pieces[opponent | PieceType::Queen]
                | self.pieces[opponent | PieceType::Bishop];

            self.clear_pinner(&mut pinner, color, king);
        });
    }

    fn clear_pinner(&mut self, pinner: &mut Bitboard, color: PieceColor, king: Tile) {
        while !pinner.is_empty() {
            let u64 = u64::from(*pinner);
            let tile = Tile::from(u64.trailing_zeros() as u8);
            self.pin_lines[color] |= self.computed.obstruction_masks.get(tile, king);
            *pinner &= u64 - 1;
        }
    }
}
