use super::{super::Bitboard, *};

impl Piece {
    fn get_index(&self) -> usize {
        match *self {
            WHITE_KING => 0,
            WHITE_QUEEN => 1,
            WHITE_ROOK => 2,
            WHITE_BISHOP => 3,
            WHITE_KNIGHT => 4,
            WHITE_PAWN => 5,
            BLACK_KING => 6,
            BLACK_QUEEN => 7,
            BLACK_ROOK => 8,
            BLACK_BISHOP => 9,
            BLACK_KNIGHT => 10,
            BLACK_PAWN => 11,
            _ => panic!("Invalid piece type"),
        }
    }
}

impl std::ops::Index<Piece> for [Bitboard; 12] {
    type Output = Bitboard;
    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece.get_index()]
    }
}

impl std::ops::IndexMut<Piece> for [Bitboard; 12] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index.get_index()]
    }
}

impl std::ops::Index<Tile> for [Option<Piece>; 64] {
    type Output = Option<Piece>;
    fn index(&self, tile: Tile) -> &Self::Output {
        &self[Into::<u8>::into(tile) as usize]
    }
}

impl std::ops::IndexMut<Tile> for [Option<Piece>; 64] {
    fn index_mut(&mut self, tile: Tile) -> &mut Self::Output {
        &mut self[Into::<u8>::into(tile) as usize]
    }
}
