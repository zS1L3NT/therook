use super::*;

impl Tile {
    pub fn get_rank(self) -> Bitboard {
        RANK_1 << (self.0 & 56)
    }

    pub fn get_file(self) -> Bitboard {
        FILE_A << (self.0 & 7)
    }

    pub fn get_diagonal(self) -> Bitboard {
        let rank = (self.0 >> 3) as i8;
        let file = (self.0 & 7) as i8;
        let left = (rank - file) * 8;

        Bitboard::from(if left >= 0 {
            u64::from(DIAGONAL_MAIN) << left
        } else {
            u64::from(DIAGONAL_MAIN) >> -left
        })
    }

    pub fn get_antidiag(self) -> Bitboard {
        let rank = (self.0 >> 3) as i8;
        let file = (self.0 & 7) as i8;
        let right = (7 - rank - file) * 8;

        Bitboard::from(if right >= 0 {
            u64::from(ANTIDIAG_MAIN) >> right
        } else {
            u64::from(ANTIDIAG_MAIN) << -right
        })
    }
}
