use super::*;

pub struct Betweens {
    data: [[Bitboard; 64]; 64],
}

impl Betweens {
    pub fn new() -> Self {
        let mut masks = Betweens {
            data: [[Bitboard::new(); 64]; 64],
        };

        // https://www.chessprogramming.org/Square_Attacked_By#Pure_Calculation
        let m1 = 1u64.wrapping_neg();
        let a2a7 = 0x0001010101010100u64;
        let b2g7 = 0x0040201008040200u64;
        let h1b7 = 0x0002040810204080u64;

        for start in 0..64u64 {
            for end in 0..64u64 {
                let between = (m1 << (start as u8)) ^ (m1 << (end as u8));
                let file = (end & 7).wrapping_sub(start & 7);
                let rank = (end | 7).wrapping_sub(start) >> 3;
                let mut line = (file & 7).wrapping_sub(1) & a2a7;
                line += 2 * ((rank & 7).wrapping_sub(1) >> 58);
                line += (rank.wrapping_sub(file) & 15).wrapping_sub(1) & b2g7;
                line += (rank.wrapping_add(file) & 15).wrapping_sub(1) & h1b7;
                line = line.wrapping_mul(between & between.wrapping_neg());

                masks.data[start as usize][end as usize] = Bitboard::from(line & between);
            }
        }

        masks
    }

    pub fn get(&self, start: u8, end: u8) -> Bitboard {
        self.data[start as usize][end as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonals() {
        let attacks = Attacks::new();
        let betweens = Betweens::new();

        for square in 0..64u8 {
            let bitboard = Bitboard::from(square);

            let reachable = attacks.get(PieceColor::White, PieceType::Rook, square, bitboard);
            let beside = attacks.get(PieceColor::White, PieceType::King, square, bitboard);

            for _square in reachable & !beside {
                if square >> 3 == _square >> 3 {
                    // Same rank
                    let mut obstructions = Bitboard::new();
                    for i in (square.min(_square) + 1)..square.max(_square) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(betweens.get(square, _square), obstructions);
                } else {
                    // Same file
                    let mut obstructions = Bitboard::new();
                    for i in ((square.min(_square) + 8)..square.max(_square)).step_by(8) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(betweens.get(square, _square), obstructions);
                }
            }
        }
    }

    #[test]
    fn diagonals() {
        let rays = Rays::new();
        let attacks = Attacks::new();
        let betweens = Betweens::new();

        for square in 0..64u8 {
            let bitboard = Bitboard::from(square);

            let reachable = attacks.get(PieceColor::White, PieceType::Bishop, square, bitboard);
            let beside = attacks.get(PieceColor::White, PieceType::King, square, bitboard);

            for _square in reachable & !beside {
                if rays.diagonals[square as usize] == rays.diagonals[_square as usize] {
                    // Same diagonal
                    let mut obstructions = Bitboard::new();
                    for i in ((square.min(_square) + 9)..square.max(_square)).step_by(9) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(betweens.get(square, _square), obstructions);
                } else {
                    // Same antidiag
                    let mut obstructions = Bitboard::new();
                    for i in ((square.min(_square) + 7)..square.max(_square)).step_by(7) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(betweens.get(square, _square), obstructions);
                }
            }
        }
    }

    #[test]
    fn unreachable() {
        let attacks = Attacks::new();
        let betweens = Betweens::new();

        let empty = Bitboard::new();

        for square in 0..64u8 {
            let bitboard = Bitboard::from(square);

            let reachable = attacks.get(PieceColor::White, PieceType::Queen, square, bitboard);
            let beside = attacks.get(PieceColor::White, PieceType::King, square, bitboard);

            for _square in !reachable | beside {
                assert_eq!(betweens.get(square, _square), empty);
            }

            for _square in reachable & !beside {
                assert_ne!(betweens.get(square, _square), empty);
            }
        }
    }
}
