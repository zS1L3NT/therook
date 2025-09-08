use super::*;

impl Board {
    pub fn update_rays(&mut self, color: PieceColor) {
        let line_masks = &self.computed.line_masks;

        self.rays[color] = Bitboard::new();

        for r#type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop] {
            for tile in self.pieces[color | r#type].get_tiles() {
                let index = u8::from(tile) as usize;

                if r#type.is_orthogonal_slider() {
                    self.rays[color] |= line_masks.ranks[index] | line_masks.files[index];
                }

                if r#type.is_diagonal_slider() {
                    self.rays[color] |= line_masks.diagonals[index] | line_masks.antidiags[index];
                }
            }
        }
    }

    pub fn update_attacks(&mut self, color: PieceColor) {
        let attack_masks = &self.computed.attack_masks;
        let occupancy = self.colors[color] | self.colors[color.opposite()];

        self.attacks[color] = Bitboard::new();

        for r#type in [
            PieceType::King,
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Pawn,
        ] {
            for tile in self.pieces[color | r#type].get_tiles() {
                self.attacks[color] |= attack_masks.get(color, r#type, tile, occupancy);
            }
        }
    }

    // https://www.chessprogramming.org/Checks_and_Pinned_Pieces_%28Bitboards%29#Absolute_Pins
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

    pub fn update_check_count(&mut self, color: PieceColor) {}
}
