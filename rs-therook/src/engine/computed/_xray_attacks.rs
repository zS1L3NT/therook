use super::*;

// https://www.chessprogramming.org/X-ray_Attacks_(Bitboards)#Modifying_Occupancy
impl Computed {
    pub fn xray_orthogonal_attacks(
        &self,
        occupancy: Bitboard,
        blockers: Bitboard,
        square: u8,
    ) -> Bitboard {
        let attacks = self
            .attacks
            .get(PieceColor::White, PieceType::Rook, square, occupancy);

        let blockers = blockers & attacks;

        if blockers.is_none() {
            blockers
        } else {
            attacks
                ^ self.attacks.get(
                    PieceColor::White,
                    PieceType::Rook,
                    square,
                    occupancy ^ blockers,
                )
        }
    }

    pub fn xray_diagonal_attacks(
        &self,
        occupancy: Bitboard,
        blockers: Bitboard,
        square: u8,
    ) -> Bitboard {
        let attacks = self
            .attacks
            .get(PieceColor::White, PieceType::Bishop, square, occupancy);

        let blockers = blockers & attacks;

        if blockers.is_none() {
            blockers
        } else {
            attacks
                ^ self.attacks.get(
                    PieceColor::White,
                    PieceType::Bishop,
                    square,
                    occupancy ^ blockers,
                )
        }
    }
}
