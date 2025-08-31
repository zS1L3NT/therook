use super::*;

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Into::<Bitboard>::into(*self).fmt(f)
    }
}
