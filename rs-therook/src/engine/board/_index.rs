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
            _ => panic!("Unknown piece type: {self:?}"),
        }
    }
}

impl<T> std::ops::Index<Piece> for [T; 12] {
    type Output = T;
    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece.get_index()]
    }
}

impl<T> std::ops::IndexMut<Piece> for [T; 12] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index.get_index()]
    }
}

impl<T> std::ops::Index<PieceColor> for [T; 2] {
    type Output = T;
    fn index(&self, color: PieceColor) -> &Self::Output {
        &self[color as usize]
    }
}

impl<T> std::ops::IndexMut<PieceColor> for [T; 2] {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        &mut self[color as usize]
    }
}

impl<T> std::ops::Index<Tile> for [T; 64] {
    type Output = T;
    fn index(&self, tile: Tile) -> &Self::Output {
        &self[u8::from(tile) as usize]
    }
}

impl<T> std::ops::IndexMut<Tile> for [T; 64] {
    fn index_mut(&mut self, tile: Tile) -> &mut Self::Output {
        &mut self[u8::from(tile) as usize]
    }
}
