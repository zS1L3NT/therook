use super::*;

pub struct LineMasks {
    pub ranks: [Bitboard; 64],
    pub files: [Bitboard; 64],
    pub diagonals: [Bitboard; 64],
    pub antidiags: [Bitboard; 64],
}

impl LineMasks {
    pub fn new() -> Self {
        let mut masks = LineMasks {
            ranks: [Bitboard::new(); 64],
            files: [Bitboard::new(); 64],
            diagonals: [Bitboard::new(); 64],
            antidiags: [Bitboard::new(); 64],
        };

        for index in 0..64u64 {
            let rank = (index >> 3) as i8;
            let file = (index & 7) as i8;

            masks.ranks[index as usize] = RANK_1 << (index & 56);

            masks.files[index as usize] = FILE_A << (index & 7);

            let left = (rank - file) * 8;
            masks.diagonals[index as usize] = Bitboard::from(if left >= 0 {
                u64::from(DIAGONAL_MAIN) << left
            } else {
                u64::from(DIAGONAL_MAIN) >> -left
            });

            let right = (7 - rank - file) * 8;
            masks.antidiags[index as usize] = Bitboard::from(if right >= 0 {
                u64::from(ANTIDIAG_MAIN) >> right
            } else {
                u64::from(ANTIDIAG_MAIN) << -right
            });
        }

        masks
    }
}
