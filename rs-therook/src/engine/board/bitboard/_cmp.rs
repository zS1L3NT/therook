use super::*;

impl std::cmp::PartialOrd for Bitboard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::cmp::PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
