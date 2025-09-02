use super::*;

#[derive(Clone, Copy)]
struct AttackMask {
    king: Bitboard,
    rank: Bitboard,
    file: Bitboard,
    diagonal: Bitboard,
    antidiag: Bitboard,
    knight: Bitboard,
    white_pawn: Bitboard,
    black_pawn: Bitboard,
}

impl AttackMask {
    pub fn new() -> Self {
        AttackMask {
            king: Bitboard::new(),
            rank: Bitboard::new(),
            file: Bitboard::new(),
            diagonal: Bitboard::new(),
            antidiag: Bitboard::new(),
            knight: Bitboard::new(),
            white_pawn: Bitboard::new(),
            black_pawn: Bitboard::new(),
        }
    }
}

pub struct AttackMasks {
    data: [AttackMask; 64],
}

impl AttackMasks {
    #[timed(AttackMasks)]
    pub fn new() -> Self {
        let mut data = [AttackMask::new(); 64];

        for index in 0..64i32 {
            let mask = &mut data[index as usize];
            let bitboard = Into::<Bitboard>::into(Into::<Tile>::into(index as u8));

            // https://www.chessprogramming.org/King_Pattern#by_Calculation

            mask.king |= bitboard.west() | bitboard | bitboard.east();
            mask.king |= mask.king.north() | mask.king.south();
            mask.king ^= bitboard;

            // https://www.chessprogramming.org/On_an_empty_Board#By_Calculation_3
            mask.rank = RANK_1 << (index & 56) as u64;
            mask.rank ^= bitboard;

            mask.file = FILE_A << (index & 7) as u64;
            mask.file ^= bitboard;

            mask.diagonal = {
                let diagonal = 8 * (index & 7) - (index & 56);
                let north = (-diagonal & (diagonal >> 31)) as u64;
                let south = (diagonal & (-diagonal >> 31)) as u64;
                (DIAGONAL_MAIN >> south) << north
            };
            mask.diagonal ^= bitboard;

            mask.antidiag = {
                let diagonal = 56 - 8 * (index & 7) - (index & 56);
                let north = (-diagonal & (diagonal >> 31)) as u64;
                let south = (diagonal & (-diagonal >> 31)) as u64;
                (ANTIDIAG_MAIN >> south) << north
            };
            mask.antidiag ^= bitboard;

            // https://www.chessprogramming.org/Knight_Pattern#Multiple_Knight_Attacks
            let east_one = bitboard.east();
            let east_two = east_one.east();
            let west_one = bitboard.west();
            let west_two = west_one.west();
            let rank_one = east_one | west_one;
            let rank_two = east_two | west_two;

            mask.knight =
                (rank_one << 16u64) | (rank_one >> 16u64) | (rank_two << 8u64) | (rank_two >> 8u64);

            // https://www.chessprogramming.org/Pawn_Attacks_(Bitboards)#Attacks_2
            mask.white_pawn = bitboard.north_east() | bitboard.north_west();
            mask.black_pawn = bitboard.south_east() | bitboard.south_west();
        }

        AttackMasks { data: data }
    }

    pub fn get(&self, piece: Piece, tile: Tile) -> Bitboard {
        let index: usize = Into::<u8>::into(tile) as usize;

        match piece.get_type() {
            PieceType::King => self.data[index].king,
            PieceType::Queen => {
                self.data[index].rank
                    | self.data[index].file
                    | self.data[index].diagonal
                    | self.data[index].antidiag
            }
            PieceType::Rook => self.data[index].rank | self.data[index].file,
            PieceType::Bishop => self.data[index].diagonal | self.data[index].antidiag,
            PieceType::Knight => self.data[index].knight,
            PieceType::Pawn => match piece.get_color() {
                PieceColor::White => self.data[index].white_pawn,
                PieceColor::Black => self.data[index].black_pawn,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use therook::{bitboard, tile};

    #[test]
    fn kings() {
        let computed = AttackMasks::new();

        assert_eq!(
            Into::<Bitboard>::into(0x3828380000u64),
            computed.get(WHITE_KING, tile!(E4))
        );

        assert_eq!(
            Into::<Bitboard>::into(0x302u64),
            computed.get(WHITE_KING, tile!(A1))
        );

        assert_eq!(
            Into::<Bitboard>::into(0x30203u64),
            computed.get(WHITE_KING, tile!(A2))
        );
    }

    #[test]
    fn queens() {
        let computed = AttackMasks::new();

        assert_eq!(
            Into::<Bitboard>::into(0x182442800284482u64) | ((FILE_E | RANK_4) ^ bitboard!(E4)),
            computed.get(WHITE_QUEEN, tile!(E4))
        );

        assert_eq!(
            Into::<Bitboard>::into(0x8040201008040200u64) | ((FILE_A | RANK_1) ^ bitboard!(A1)),
            computed.get(WHITE_QUEEN, tile!(A1))
        );
    }

    #[test]
    fn rooks() {
        let computed = AttackMasks::new();

        assert_eq!(
            (FILE_E | RANK_4) ^ bitboard!(E4),
            computed.get(WHITE_ROOK, tile!(E4))
        );

        assert_eq!(
            (FILE_A | RANK_1) ^ bitboard!(A1),
            computed.get(WHITE_ROOK, tile!(A1))
        );
    }

    #[test]
    fn bishops() {
        let computed = AttackMasks::new();

        assert_eq!(
            Into::<Bitboard>::into(0x182442800284482u64),
            computed.get(WHITE_BISHOP, tile!(E4))
        );

        assert_eq!(
            Into::<Bitboard>::into(0x4020100804020002u64),
            computed.get(WHITE_BISHOP, tile!(A2))
        );

        assert_eq!(
            Into::<Bitboard>::into(0x8040201008040200u64),
            computed.get(WHITE_BISHOP, tile!(A1))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x2040810204080u64),
            computed.get(WHITE_BISHOP, tile!(A8))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x102040810204000u64),
            computed.get(WHITE_BISHOP, tile!(H1))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x40201008040201u64),
            computed.get(WHITE_BISHOP, tile!(H8))
        );
    }

    #[test]
    fn knights() {
        let computed = AttackMasks::new();

        assert_eq!(
            0x284400442800u64,
            computed.get(WHITE_KNIGHT, tile!(E4)).into()
        );

        assert_eq!(
            Into::<Bitboard>::into(0x20400u64),
            computed.get(WHITE_KNIGHT, tile!(A1))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x2040004u64),
            computed.get(WHITE_KNIGHT, tile!(A2))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x204000402u64),
            computed.get(WHITE_KNIGHT, tile!(A3))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x50800u64),
            computed.get(WHITE_KNIGHT, tile!(B1))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x5080008u64),
            computed.get(WHITE_KNIGHT, tile!(B2))
        );
        assert_eq!(
            Into::<Bitboard>::into(0x508000805u64),
            computed.get(WHITE_KNIGHT, tile!(B3))
        );
        assert_eq!(
            Into::<Bitboard>::into(0xA1100u64),
            computed.get(WHITE_KNIGHT, tile!(C1))
        );
        assert_eq!(
            Into::<Bitboard>::into(0xA110011u64),
            computed.get(WHITE_KNIGHT, tile!(C2))
        );
        assert_eq!(
            Into::<Bitboard>::into(0xA1100110au64),
            computed.get(WHITE_KNIGHT, tile!(C3))
        );
    }

    #[test]
    fn pawns() {
        let computed = AttackMasks::new();

        assert_eq!(
            Into::<Bitboard>::into(0x280000),
            computed.get(WHITE_PAWN, tile!(E2))
        );

        assert_eq!(
            Into::<Bitboard>::into(0x280000000000),
            computed.get(BLACK_PAWN, tile!(E7))
        )
    }
}
