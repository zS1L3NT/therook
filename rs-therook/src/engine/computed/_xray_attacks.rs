use super::*;

// https://www.chessprogramming.org/X-ray_Attacks_(Bitboards)#Modifying_Occupancy
impl Computed {
    pub fn xray_orthogonal_attacks(
        &self,
        occupancy: Bitboard,
        blockers: Bitboard,
        square: u8,
    ) -> Bitboard {
        let possible_attackers =
            self.attacks
                .get(PieceColor::White, PieceType::Rook, square, occupancy);

        let pinned = blockers & possible_attackers;

        if pinned.is_some() {
            let possible_attackers_without_pinned = self.attacks.get(
                PieceColor::White,
                PieceType::Rook,
                square,
                occupancy ^ pinned,
            );

            possible_attackers ^ possible_attackers_without_pinned
        } else {
            Bitboard::new()
        }
    }

    pub fn xray_diagonal_attacks(
        &self,
        occupancy: Bitboard,
        blockers: Bitboard,
        square: u8,
    ) -> Bitboard {
        let possible_attackers =
            self.attacks
                .get(PieceColor::White, PieceType::Bishop, square, occupancy);

        let pinned = blockers & possible_attackers;

        if pinned.is_some() {
            let possible_attackers_without_pinned = self.attacks.get(
                PieceColor::White,
                PieceType::Bishop,
                square,
                occupancy ^ pinned,
            );

            possible_attackers ^ possible_attackers_without_pinned
        } else {
            Bitboard::new()
        }
    }
}
