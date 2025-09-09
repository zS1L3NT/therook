use super::*;
use std::num::Wrapping;

pub struct Attacks {
    rays: Rays,

    kings: [Bitboard; 64],
    knights: [Bitboard; 64],
    white_pawns: [Bitboard; 64],
    black_pawns: [Bitboard; 64],

    ranks: [[Bitboard; 64]; 64],
    files: [[Bitboard; 64]; 64],
    diagonals: [[Bitboard; 64]; 64],
    antidiags: [[Bitboard; 64]; 64],
}

impl Attacks {
    const FILE_B: u64 = 0x0202020202020202;

    pub fn new() -> Self {
        let mut masks = Attacks {
            rays: Rays::new(),

            kings: [Bitboard::new(); 64],
            knights: [Bitboard::new(); 64],
            white_pawns: [Bitboard::new(); 64],
            black_pawns: [Bitboard::new(); 64],

            ranks: [[Bitboard::new(); 64]; 64],
            files: [[Bitboard::new(); 64]; 64],
            diagonals: [[Bitboard::new(); 64]; 64],
            antidiags: [[Bitboard::new(); 64]; 64],
        };

        for index in 0..64usize {
            let bitboard = Bitboard::from(index as u8);

            // https://www.chessprogramming.org/King_Pattern#by_Calculation
            masks.kings[index] |= bitboard.west() | bitboard | bitboard.east();
            masks.kings[index] |= masks.kings[index].north() | masks.kings[index].south();
            masks.kings[index] ^= bitboard;

            // https://www.chessprogramming.org/Knight_Pattern#Multiple_Knight_Attacks
            let east_one = bitboard.east();
            let east_two = east_one.east();
            let west_one = bitboard.west();
            let west_two = west_one.west();
            let rank_one = east_one | west_one;
            let rank_two = east_two | west_two;

            masks.knights[index] =
                (rank_one << 16) | (rank_one >> 16) | (rank_two << 8) | (rank_two >> 8);

            // https://www.chessprogramming.org/Pawn_Attacks_(Bitboards)#Attacks_2
            masks.white_pawns[index] = bitboard.north_east() | bitboard.north_west();
            masks.black_pawns[index] = bitboard.south_east() | bitboard.south_west();

            let rank = index >> 3;
            let file = index & 7;
            let slider = 1 << file;
            for occupancy in 0..64 {
                // https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks#Lookup_Techniques
                let o = Wrapping((occupancy as u8) << 1);
                let s = Wrapping(slider as u8);
                let _2 = Wrapping(2u8);

                // https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks#Sliding_Attacks_by_Calculation
                let single_rank = ((o - _2 * s)
                    ^ (o.reverse_bits() - _2 * s.reverse_bits()).reverse_bits())
                .0 as u64;
                let single_rank_files = single_rank.wrapping_mul(FILE_A.into());
                let single_file_ranks = u64::from(Bitboard::from(single_rank_files).clockwise());

                masks.ranks[index][occupancy] = masks.rays.ranks[index] & single_rank_files;

                // Generated through 2 days of blood sweat and tears worth of testing
                let file_index = rank + ((7 - file) * 8);

                masks.files[file_index][occupancy] =
                    (FILE_A << (index as u8 >> 3)) & single_file_ranks;

                // https://www.chessprogramming.org/On_an_empty_Board#By_Calculation_3
                masks.diagonals[index][occupancy] = {
                    let diagonal = 8 * (index & 7) as i32 - (index & 56) as i32;
                    let north = (-diagonal & (diagonal >> 31)) as u8;
                    let south = (diagonal & (-diagonal >> 31)) as u8;
                    ((DIAGONAL_MAIN >> south) << north) & single_rank_files
                };

                masks.antidiags[index][occupancy] = {
                    let diagonal = 56 - 8 * (index & 7) as i32 - (index & 56) as i32;
                    let north = (-diagonal & (diagonal >> 31)) as u8;
                    let south = (diagonal & (-diagonal >> 31)) as u8;
                    ((ANTIDIAG_MAIN >> south) << north) & single_rank_files
                };
            }
        }

        masks
    }

    pub fn get(
        &self,
        color: PieceColor,
        r#type: PieceType,
        tile: Tile,
        occupancy: Bitboard,
    ) -> Bitboard {
        let index = u8::from(tile) as usize;

        match r#type {
            PieceType::King => self.kings[index],
            PieceType::Queen => {
                let rank = occupancy & self.rays.ranks[index];
                let file = occupancy & self.rays.files[index];
                let file = file.anticlockwise();
                let diagonal = occupancy & self.rays.diagonals[index];
                let antidiag = occupancy & self.rays.antidiags[index];

                self.ranks[index][Self::kindergarten(rank)]
                    | self.files[index][Self::kindergarten(file)]
                    | self.diagonals[index][Self::kindergarten(diagonal)]
                    | self.antidiags[index][Self::kindergarten(antidiag)]
            }
            PieceType::Rook => {
                let rank = occupancy & self.rays.ranks[index];
                let file = occupancy & self.rays.files[index];
                let file = file.anticlockwise();

                self.ranks[index][Self::kindergarten(rank)]
                    | self.files[index][Self::kindergarten(file)]
            }
            PieceType::Bishop => {
                let diagonal = occupancy & self.rays.diagonals[index];
                let antidiag = occupancy & self.rays.antidiags[index];

                self.diagonals[index][Self::kindergarten(diagonal)]
                    | self.antidiags[index][Self::kindergarten(antidiag)]
            }
            PieceType::Knight => self.knights[index],
            PieceType::Pawn => match color {
                PieceColor::White => self.white_pawns[index],
                PieceColor::Black => self.black_pawns[index],
            },
        }
    }

    // https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks#Kindergarten_Bitboards
    fn kindergarten(bitboard: Bitboard) -> usize {
        (u64::from(bitboard).wrapping_mul(Self::FILE_B) >> 58) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn kings() {
        let masks = Attacks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);
            let rank = index >> 3;
            let file = index & 7;

            let mut expected = Bitboard::new();

            if rank != 7 {
                expected |= bitboard << 8;
            }

            if rank != 7 && file != 7 {
                expected |= bitboard << 9;
            }

            if file != 7 {
                expected |= bitboard << 1;
            }

            if rank != 0 && file != 7 {
                expected |= bitboard >> 7;
            }

            if rank != 0 {
                expected |= bitboard >> 8;
            }

            if rank != 0 && file != 0 {
                expected |= bitboard >> 9;
            }

            if file != 0 {
                expected |= bitboard >> 1;
            }

            if rank != 7 && file != 0 {
                expected |= bitboard << 7;
            }

            assert_eq!(
                masks.get(PieceColor::White, PieceType::King, tile, bitboard),
                expected
            );
        }
    }

    #[test]
    fn knights() {
        let masks = Attacks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);
            let rank = index >> 3;
            let file = index & 7;

            let mut expected = Bitboard::new();

            if rank < 6 && file > 0 {
                expected |= bitboard << 15;
            }

            if rank < 6 && file < 7 {
                expected |= bitboard << 17;
            }

            if rank < 7 && file < 6 {
                expected |= bitboard << 10;
            }

            if rank > 0 && file < 6 {
                expected |= bitboard >> 6;
            }

            if rank > 1 && file < 7 {
                expected |= bitboard >> 15;
            }

            if rank > 1 && file > 0 {
                expected |= bitboard >> 17;
            }

            if rank > 0 && file > 1 {
                expected |= bitboard >> 10;
            }

            if rank < 7 && file > 1 {
                expected |= bitboard << 6;
            }

            assert_eq!(
                masks.get(PieceColor::White, PieceType::Knight, tile, bitboard),
                expected
            );
        }
    }

    #[test]
    fn white_pawns() {
        let masks = Attacks::new();

        for index in 8..55 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);
            let file = index & 7;

            let mut expected = Bitboard::new();
            if file != 0 {
                expected |= bitboard << 7;
            }

            if file != 7 {
                expected |= bitboard << 9
            }

            assert_eq!(
                masks.get(PieceColor::White, PieceType::Pawn, tile, bitboard),
                expected
            );
        }
    }

    #[test]
    fn black_pawns() {
        let masks = Attacks::new();

        for index in 8..55 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);

            let mut expected = Bitboard::new();
            if index & 7 != 0 {
                expected |= bitboard >> 9;
            }

            if index & 7 != 7 {
                expected |= bitboard >> 7
            }

            assert_eq!(
                masks.get(PieceColor::Black, PieceType::Pawn, tile, bitboard),
                expected
            );
        }
    }

    #[test]
    fn ranks_files_alone() {
        let masks = Attacks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);
            assert_eq!(
                (masks.rays.ranks[index as usize] | masks.rays.files[index as usize]) ^ bitboard,
                masks.get(PieceColor::White, PieceType::Rook, tile, bitboard)
            );
        }
    }

    #[test]
    fn ranks_files_with_pieces() {
        let masks = Attacks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);

            for rank_occupancy in 0..256u64 {
                if rank_occupancy & 1 << (7 - (index >> 3)) == 0 {
                    continue;
                }

                for file_occupancy in 0..256u64 {
                    if file_occupancy & 1 << (index & 7) == 0 {
                        continue;
                    }

                    let occupancy = Bitboard::from(rank_occupancy) << (index & 56)
                        | Bitboard::from(file_occupancy).clockwise() << (index & 7);

                    assert_eq!(
                        masks.get(PieceColor::White, PieceType::Rook, tile, occupancy),
                        walk_directions(vec![8, 1, -8, -1], tile, occupancy)
                    );
                }
            }
        }
    }

    #[test]
    fn diagonals_antidiags_alone() {
        let masks = Attacks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);
            assert_eq!(
                (masks.rays.diagonals[index as usize] | masks.rays.antidiags[index as usize])
                    ^ bitboard,
                masks.get(PieceColor::White, PieceType::Bishop, tile, bitboard)
            );
        }
    }

    #[test]
    fn diagonals_antidiags_with_pieces() {
        let masks = Attacks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);

            let diagonal = masks.rays.diagonals[index as usize];
            let diagonal_occupancies = (1..=diagonal.count())
                .flat_map(|l| diagonal.into_iter().combinations(l))
                .map(|ts| {
                    ts.iter()
                        .fold(Bitboard::new(), |acc, el| acc | Bitboard::from(*el))
                })
                .filter(|b| (*b & bitboard).is_some())
                .collect::<Vec<_>>();

            let antidiag = masks.rays.antidiags[index as usize];
            let antidiag_occupancies = (1..=antidiag.count())
                .flat_map(|l| antidiag.into_iter().combinations(l))
                .map(|ts| {
                    ts.iter()
                        .fold(Bitboard::new(), |acc, el| acc | Bitboard::from(*el))
                })
                .filter(|b| (*b & bitboard).is_some())
                .collect::<Vec<_>>();

            for diagonal_occupancy in &diagonal_occupancies {
                for antidiag_occupancy in &antidiag_occupancies {
                    let occupancy = (*diagonal_occupancy | *antidiag_occupancy) ^ bitboard;

                    assert_eq!(
                        masks.get(PieceColor::White, PieceType::Bishop, tile, occupancy),
                        walk_directions(vec![7, 9, -7, -9], tile, occupancy),
                    );
                }
            }
        }
    }

    fn walk_directions(directions: Vec<i8>, tile: Tile, occupancy: Bitboard) -> Bitboard {
        let mut expected = Bitboard::new();

        for direction in directions {
            let mut target_index = u8::from(tile) as i8;
            let mut target_rank = target_index >> 3;
            let mut target_file = target_index & 7;

            while !will_leave_board(direction, target_rank, target_file) {
                target_index += direction;
                target_rank = target_index >> 3;
                target_file = target_index & 7;

                let target_bitboard = Bitboard::from(target_index as u8);

                expected |= target_bitboard;

                if (occupancy & target_bitboard).is_some() {
                    break;
                }
            }
        }

        expected
    }

    fn will_leave_board(direction: i8, rank: i8, file: i8) -> bool {
        match direction {
            8 => rank == 7,
            9 => rank == 7 || file == 7,
            1 => file == 7,
            -7 => rank == 0 || file == 7,
            -8 => rank == 0,
            -9 => rank == 0 || file == 0,
            -1 => file == 0,
            7 => rank == 7 || file == 0,
            _ => unreachable!(),
        }
    }
}
