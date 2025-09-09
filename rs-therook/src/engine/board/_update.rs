use super::*;

impl Board {
    pub fn update_rays(&mut self, color: PieceColor) {
        self.rays[color] = Bitboard::new();

        for r#type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop] {
            for index in self.pieces[color | r#type] {
                if r#type.is_orthogonal_slider() {
                    self.rays[color] |= self.computed.rays.ranks[index as usize];
                    self.rays[color] |= self.computed.rays.files[index as usize];
                }

                if r#type.is_diagonal_slider() {
                    self.rays[color] |= self.computed.rays.diagonals[index as usize];
                    self.rays[color] |= self.computed.rays.antidiags[index as usize];
                }
            }
        }
    }

    pub fn update_attacks(&mut self, color: PieceColor) {
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
            for index in self.pieces[color | r#type] {
                self.attacks[color] |=
                    self.computed
                        .attacks
                        .get(color, r#type, Tile::from(index), occupancy);
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

            let king = Tile::try_from(self.pieces[color | PieceType::King]).unwrap();

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
        while pinner.is_some() {
            let u64 = u64::from(*pinner);
            let tile = Tile::from(u64.trailing_zeros() as u8);
            self.pin_lines[color] |= self.computed.betweens.get(tile, king);
            *pinner &= u64 - 1;
        }
    }

    pub fn update_check_count(&mut self, color: PieceColor) {}
}
