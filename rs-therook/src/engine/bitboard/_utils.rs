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
        self << 8
    }

    pub fn north_east(self) -> Bitboard {
        (self << 9) & !FILE_A
    }

    pub fn east(self) -> Bitboard {
        (self << 1) & !FILE_A
    }

    pub fn south_east(self) -> Bitboard {
        (self >> 7) & !FILE_A
    }

    pub fn south(self) -> Bitboard {
        self >> 8
    }

    pub fn south_west(self) -> Bitboard {
        (self >> 9) & !FILE_H
    }

    pub fn west(self) -> Bitboard {
        (self >> 1) & !FILE_H
    }

    pub fn north_west(self) -> Bitboard {
        (self << 7) & !FILE_H
    }

    pub fn clockwise(self) -> Bitboard {
        self.flip_diagonal_a1_h8().flip_vertical()
    }

    pub fn anticlockwise(self) -> Bitboard {
        self.flip_vertical().flip_diagonal_a1_h8()
    }

    fn flip_diagonal_a1_h8(self) -> Bitboard {
        let k1 = 0x5500550055005500u64;
        let k2 = 0x3333000033330000u64;
        let k4 = 0x0f0f0f0f00000000u64;
        let mut x = self.0;
        let mut t = k4 & (x ^ (x << 28));
        x ^= t ^ (t >> 28);
        t = k2 & (x ^ (x << 14));
        x ^= t ^ (t >> 14);
        t = k1 & (x ^ (x << 7));
        x ^= t ^ (t >> 7);
        x.into()
    }

    fn flip_vertical(self) -> Bitboard {
        let k1 = 0x00FF00FF00FF00FFu64;
        let k2 = 0x0000FFFF0000FFFFu64;
        let mut x = self.0;
        x = ((x >> 8) & k1) | ((x & k1) << 8);
        x = ((x >> 16) & k2) | ((x & k2) << 16);
        x = (x >> 32) | (x << 32);
        x.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clockwise() {
        assert_eq!(
            Bitboard::from(0xFF888C92610000u64),
            Bitboard::from(0x1E2222120E0A1222u64).clockwise()
        );
    }

    #[test]
    fn anticlockwise() {
        assert_eq!(
            Bitboard::from(0x86493111FF00u64),
            Bitboard::from(0x1E2222120E0A1222u64).anticlockwise()
        );
    }
}
