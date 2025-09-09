use super::*;

// https://www.chessprogramming.org/X-ray_Attacks_(Bitboards)#Modifying_Occupancy
impl Computed {
    pub fn xray_orthogonal_attacks(
        &self,
        occupancy: Bitboard,
        blockers: Bitboard,
        tile: Tile,
    ) -> Bitboard {
        let attacks = self
            .attacks
            .get(PieceColor::White, PieceType::Rook, tile, occupancy);

        let blockers = blockers & attacks;

        if blockers.is_none() {
            blockers
        } else {
            attacks
                ^ self.attacks.get(
                    PieceColor::White,
                    PieceType::Rook,
                    tile,
                    occupancy ^ blockers,
                )
        }
    }

    pub fn xray_diagonal_attacks(
        &self,
        occupancy: Bitboard,
        blockers: Bitboard,
        tile: Tile,
    ) -> Bitboard {
        let attacks = self
            .attacks
            .get(PieceColor::White, PieceType::Bishop, tile, occupancy);

        let blockers = blockers & attacks;

        if blockers.is_none() {
            blockers
        } else {
            attacks
                ^ self.attacks.get(
                    PieceColor::White,
                    PieceType::Bishop,
                    tile,
                    occupancy ^ blockers,
                )
        }
    }
}
