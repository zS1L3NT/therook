#![allow(dead_code)]

use super::*;

static RANK: u64 = 0xFF;
pub const RANK_1: Bitboard = Bitboard(RANK << 8 * 0);
pub const RANK_2: Bitboard = Bitboard(RANK << 8 * 1);
pub const RANK_3: Bitboard = Bitboard(RANK << 8 * 2);
pub const RANK_4: Bitboard = Bitboard(RANK << 8 * 3);
pub const RANK_5: Bitboard = Bitboard(RANK << 8 * 4);
pub const RANK_6: Bitboard = Bitboard(RANK << 8 * 5);
pub const RANK_7: Bitboard = Bitboard(RANK << 8 * 6);
pub const RANK_8: Bitboard = Bitboard(RANK << 8 * 7);

static FILE: u64 = 0x101010101010101;
pub const FILE_A: Bitboard = Bitboard(FILE * 1 << 0);
pub const FILE_B: Bitboard = Bitboard(FILE * 1 << 1);
pub const FILE_C: Bitboard = Bitboard(FILE * 1 << 2);
pub const FILE_D: Bitboard = Bitboard(FILE * 1 << 3);
pub const FILE_E: Bitboard = Bitboard(FILE * 1 << 4);
pub const FILE_F: Bitboard = Bitboard(FILE * 1 << 5);
pub const FILE_G: Bitboard = Bitboard(FILE * 1 << 6);
pub const FILE_H: Bitboard = Bitboard(FILE * 1 << 7);
