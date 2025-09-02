use super::*;

static RANK: u64 = 0xFF;
pub const RANK_1: Bitboard = Bitboard(RANK << 8 * 0);
pub const RANK_2: Bitboard = Bitboard(RANK << 8 * 1);
#[allow(dead_code)]
pub const RANK_3: Bitboard = Bitboard(RANK << 8 * 2);
#[allow(dead_code)]
pub const RANK_4: Bitboard = Bitboard(RANK << 8 * 3);
#[allow(dead_code)]
pub const RANK_5: Bitboard = Bitboard(RANK << 8 * 4);
#[allow(dead_code)]
pub const RANK_6: Bitboard = Bitboard(RANK << 8 * 5);
pub const RANK_7: Bitboard = Bitboard(RANK << 8 * 6);
pub const RANK_8: Bitboard = Bitboard(RANK << 8 * 7);

static FILE: u64 = 0x0101010101010101;
pub const FILE_A: Bitboard = Bitboard(FILE * 1 << 0);
pub const FILE_B: Bitboard = Bitboard(FILE * 1 << 1);
pub const FILE_C: Bitboard = Bitboard(FILE * 1 << 2);
pub const FILE_D: Bitboard = Bitboard(FILE * 1 << 3);
pub const FILE_E: Bitboard = Bitboard(FILE * 1 << 4);
pub const FILE_F: Bitboard = Bitboard(FILE * 1 << 5);
pub const FILE_G: Bitboard = Bitboard(FILE * 1 << 6);
pub const FILE_H: Bitboard = Bitboard(FILE * 1 << 7);

pub const DIAGONAL_MAIN: Bitboard = Bitboard(0x8040201008040201);
pub const ANTIDIAG_MAIN: Bitboard = Bitboard(0x0102040810204080);

impl Bitboard {
    pub fn north(self) -> Bitboard {
        self << 8u64
    }

    pub fn north_east(self) -> Bitboard {
        (self << 9u64) & !FILE_A
    }

    pub fn east(self) -> Bitboard {
        (self << 1u64) & !FILE_A
    }

    pub fn south_east(self) -> Bitboard {
        (self >> 7u64) & !FILE_A
    }

    pub fn south(self) -> Bitboard {
        self >> 8u64
    }

    pub fn south_west(self) -> Bitboard {
        (self >> 9u64) & !FILE_H
    }

    pub fn west(self) -> Bitboard {
        (self >> 1u64) & !FILE_H
    }

    pub fn north_west(self) -> Bitboard {
        (self << 7u64) & !FILE_H
    }
}
