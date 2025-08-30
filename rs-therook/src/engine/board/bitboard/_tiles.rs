#![allow(dead_code)]

use super::*;

static RANK: u64 = 0xFF;
pub static RANK_1: Bitboard = Bitboard(RANK << 8 * 0);
pub static RANK_2: Bitboard = Bitboard(RANK << 8 * 1);
pub static RANK_3: Bitboard = Bitboard(RANK << 8 * 2);
pub static RANK_4: Bitboard = Bitboard(RANK << 8 * 3);
pub static RANK_5: Bitboard = Bitboard(RANK << 8 * 4);
pub static RANK_6: Bitboard = Bitboard(RANK << 8 * 5);
pub static RANK_7: Bitboard = Bitboard(RANK << 8 * 6);
pub static RANK_8: Bitboard = Bitboard(RANK << 8 * 7);

static FILE: u64 = 0x101010101010101;
pub static FILE_A: Bitboard = Bitboard(FILE * 1 << 0);
pub static FILE_B: Bitboard = Bitboard(FILE * 1 << 1);
pub static FILE_C: Bitboard = Bitboard(FILE * 1 << 2);
pub static FILE_D: Bitboard = Bitboard(FILE * 1 << 3);
pub static FILE_E: Bitboard = Bitboard(FILE * 1 << 4);
pub static FILE_F: Bitboard = Bitboard(FILE * 1 << 5);
pub static FILE_G: Bitboard = Bitboard(FILE * 1 << 6);
pub static FILE_H: Bitboard = Bitboard(FILE * 1 << 7);
