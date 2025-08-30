use super::*;

impl std::ops::Index<Piece> for [Bitboard; 12] {
    type Output = Bitboard;
    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Piece> for [Bitboard; 12] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
