use super::*;

impl Piece {
    fn get_pieces_index(&self) -> usize {
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

    fn get_castle_index(&self) -> usize {
        match *self {
            WHITE_KING => 0,
            WHITE_QUEEN => 1,
            BLACK_KING => 2,
            BLACK_QUEEN => 3,
            _ => panic!("Unknown castling type: {self:?}"),
        }
    }
}

impl<T> std::ops::Index<Piece> for [T; 12] {
    type Output = T;
    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece.get_pieces_index()]
    }
}

impl<T> std::ops::IndexMut<Piece> for [T; 12] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index.get_pieces_index()]
    }
}

impl std::ops::Index<Piece> for [bool; 4] {
    type Output = bool;
    fn index(&self, index: Piece) -> &Self::Output {
        &self[index.get_castle_index()]
    }
}

impl std::ops::IndexMut<Piece> for [bool; 4] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index.get_castle_index()]
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
