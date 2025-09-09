use super::*;

impl Iterator for Bitboard {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let square = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Some(square)
    }
}
