use super::*;

impl Board {
    pub fn update_rays(&mut self, color: PieceColor) {
        let mut bitboard = Bitboard::new();

        for r#type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop] {
            for square in self.pieces[color | r#type] {
                if r#type.is_orthogonal_slider() {
                    bitboard |= self.computed.rays.ranks[square as usize];
                    bitboard |= self.computed.rays.files[square as usize];
                }

                if r#type.is_diagonal_slider() {
                    bitboard |= self.computed.rays.diagonals[square as usize];
                    bitboard |= self.computed.rays.antidiags[square as usize];
                }
            }
        }

        if bitboard == self.rays[color] {
            log::warn!("Board::update_rays() called but rays didn't change");
        } else {
            self.rays[color] = bitboard;
        }
    }

    pub fn update_attacks(&mut self, color: PieceColor) {
        let mut bitboard = Bitboard::new();

        let enemy = color.opposite();
        let enemy_king = self.pieces[enemy | PieceType::King];
        let occupancy = self.colors[color] | self.colors[enemy];

        for r#type in [
            PieceType::King,
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Pawn,
        ] {
            for square in self.pieces[color | r#type] {
                let attacks = self.computed.attacks.get(color, r#type, square, occupancy);

                bitboard |= attacks;

                if (attacks & enemy_king).is_some() {
                    self.check_state[enemy] = match self.check_state[enemy] {
                        CheckState::None => CheckState::Single(square),
                        CheckState::Single(_) => CheckState::Double,
                        CheckState::Double => unreachable!(),
                    }
                }
            }
        }

        if bitboard == self.attacks[color] {
            log::warn!("Board::update_attacks() called but attacks didn't change");
        } else {
            self.attacks[color] = bitboard;
        }
    }

    // https://www.chessprogramming.org/Checks_and_Pinned_Pieces_%28Bitboards%29#Absolute_Pins
    pub fn update_pin_lines(&mut self, color: PieceColor) {
        let mut bitboard = Bitboard::new();

        let enemy = color.opposite();
        let king_square = u8::try_from(self.pieces[color | PieceType::King]).unwrap();

        let friendlies = self.colors[color];
        let enemies = self.colors[enemy];
        let occupancy = friendlies | enemies;

        let mut pinner = self
            .computed
            .xray_orthogonal_attacks(occupancy, friendlies, king_square);
        pinner &= self.pieces[enemy | PieceType::Queen] | self.pieces[enemy | PieceType::Rook];

        self.clear_pinner(&mut bitboard, &mut pinner, king_square);

        let mut pinner = self
            .computed
            .xray_diagonal_attacks(occupancy, friendlies, king_square);
        pinner &= self.pieces[enemy | PieceType::Queen] | self.pieces[enemy | PieceType::Bishop];

        self.clear_pinner(&mut bitboard, &mut pinner, king_square);

        if bitboard == self.pin_lines[color] {
            log::warn!("Board::update_pin_lines() called but pin lines didn't change");
        } else {
            self.pin_lines[color] = bitboard;
        }
    }

    fn clear_pinner(&mut self, bitboard: &mut Bitboard, pinner: &mut Bitboard, king_square: u8) {
        while pinner.is_some() {
            let u64 = u64::from(*pinner);
            *bitboard |= self
                .computed
                .betweens
                .get(u64.trailing_zeros() as u8, king_square);
            *pinner &= u64 - 1;
        }
    }
}
