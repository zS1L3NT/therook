use super::*;

pub struct Rays {
    pub ranks: [Bitboard; 64],
    pub files: [Bitboard; 64],
    pub diagonals: [Bitboard; 64],
    pub antidiags: [Bitboard; 64],
}

impl Rays {
    pub fn new() -> Self {
        let mut masks = Rays {
            ranks: [Bitboard::new(); 64],
            files: [Bitboard::new(); 64],
            diagonals: [Bitboard::new(); 64],
            antidiags: [Bitboard::new(); 64],
        };

        for square in 0..64 {
            let rank = (square >> 3) as i8;
            let file = (square & 7) as i8;

            masks.ranks[square as usize] = RANK_1 << (square & 56);

            masks.files[square as usize] = FILE_A << (square & 7);

            let left = (rank - file) * 8;
            masks.diagonals[square as usize] = Bitboard::from(if left >= 0 {
                u64::from(DIAGONAL_MAIN) << left
            } else {
                u64::from(DIAGONAL_MAIN) >> -left
            });

            let right = (7 - rank - file) * 8;
            masks.antidiags[square as usize] = Bitboard::from(if right >= 0 {
                u64::from(ANTIDIAG_MAIN) >> right
            } else {
                u64::from(ANTIDIAG_MAIN) << -right
            });
        }

        masks
    }
}
