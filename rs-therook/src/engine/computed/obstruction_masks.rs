use super::*;

pub struct ObstructionMasks {
    data: [[Bitboard; 64]; 64],
}

impl ObstructionMasks {
    pub fn new() -> Self {
        let mut masks = ObstructionMasks {
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

    pub fn get(&self, start: Tile, end: Tile) -> Bitboard {
        self.data[u8::from(start) as usize][u8::from(end) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orthogonals() {
        let attack_masks = AttackMasks::new();
        let obstruction_masks = ObstructionMasks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);

            let reachable = attack_masks.get(PieceColor::White, PieceType::Rook, tile, bitboard);
            let beside = attack_masks.get(PieceColor::White, PieceType::King, tile, bitboard);

            for _index in reachable & !beside {
                let _tile = Tile::from(_index);

                if index >> 3 == _index >> 3 {
                    // Same rank
                    let mut obstructions = Bitboard::new();
                    for i in (index.min(_index) + 1)..(index.max(_index)) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(obstruction_masks.get(tile, _tile), obstructions);
                } else {
                    // Same file
                    let mut obstructions = Bitboard::new();
                    for i in ((index.min(_index) + 8)..index.max(_index)).step_by(8) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(obstruction_masks.get(tile, _tile), obstructions);
                }
            }
        }
    }

    #[test]
    fn diagonals() {
        let line_masks = LineMasks::new();
        let attack_masks = AttackMasks::new();
        let obstruction_masks = ObstructionMasks::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);

            let reachable = attack_masks.get(PieceColor::White, PieceType::Bishop, tile, bitboard);
            let beside = attack_masks.get(PieceColor::White, PieceType::King, tile, bitboard);

            for _index in reachable & !beside {
                let _tile = Tile::from(_index);

                if line_masks.diagonals[index as usize] == line_masks.diagonals[_index as usize] {
                    // Same diagonal
                    let mut obstructions = Bitboard::new();
                    for i in ((index.min(_index) + 9)..index.max(_index)).step_by(9) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(obstruction_masks.get(tile, _tile), obstructions);
                } else {
                    // Same antidiag
                    let mut obstructions = Bitboard::new();
                    for i in ((index.min(_index) + 7)..index.max(_index)).step_by(7) {
                        obstructions |= 1u64 << i;
                    }

                    assert_eq!(obstruction_masks.get(tile, _tile), obstructions);
                }
            }
        }
    }

    #[test]
    fn unreachable() {
        let attack_masks = AttackMasks::new();
        let obstruction_masks = ObstructionMasks::new();

        let empty = Bitboard::new();

        for index in 0..64u8 {
            let tile = Tile::from(index);
            let bitboard = Bitboard::from(index);

            let reachable = attack_masks.get(PieceColor::White, PieceType::Queen, tile, bitboard);
            let beside = attack_masks.get(PieceColor::White, PieceType::King, tile, bitboard);

            for _index in !reachable | beside {
                assert_eq!(obstruction_masks.get(tile, Tile::from(_index)), empty);
            }

            for _index in reachable & !beside {
                assert_ne!(obstruction_masks.get(tile, Tile::from(_index)), empty);
            }
        }
    }
}
