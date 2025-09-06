mod _bit;

use CastleDirection::*;

pub struct Castling(u8);

pub enum CastleDirection {
    WhiteKing = 1 << 7,
    WhiteQueen = 1 << 6,
    BlackKing = 1 << 5,
    BlackQueen = 1 << 4,
}

impl Castling {
    pub fn new() -> Self {
        Castling(0)
    }

    pub fn initial() -> Self {
        Castling(WhiteKing as u8 | WhiteQueen as u8 | BlackKing as u8 | BlackQueen as u8)
    }

    pub fn can(&self, direction: CastleDirection) -> bool {
        self.0 & direction as u8 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn none() {
        assert!(!Castling::new().can(WhiteKing));
        assert!(!Castling::new().can(WhiteQueen));
        assert!(!Castling::new().can(BlackKing));
        assert!(!Castling::new().can(BlackQueen));
    }

    #[test]
    pub fn white_king() {
        assert!(Castling(0b10000000).can(WhiteKing));

        assert!(!Castling(0b10000000).can(WhiteQueen));
        assert!(!Castling(0b10000000).can(BlackKing));
        assert!(!Castling(0b10000000).can(BlackQueen));
    }

    #[test]
    pub fn white_queen() {
        assert!(Castling(0b01000000).can(WhiteQueen));

        assert!(!Castling(0b01000000).can(WhiteKing));
        assert!(!Castling(0b01000000).can(BlackKing));
        assert!(!Castling(0b01000000).can(BlackQueen));
    }

    #[test]
    pub fn black_king() {
        assert!(Castling(0b00100000).can(BlackKing));

        assert!(!Castling(0b00100000).can(WhiteKing));
        assert!(!Castling(0b00100000).can(WhiteQueen));
        assert!(!Castling(0b00100000).can(BlackQueen));
    }

    #[test]
    pub fn black_queen() {
        assert!(Castling(0b00010000).can(BlackQueen));

        assert!(!Castling(0b00010000).can(WhiteKing));
        assert!(!Castling(0b00010000).can(WhiteQueen));
        assert!(!Castling(0b00010000).can(BlackKing));
    }

    #[test]
    pub fn white_both() {
        assert!(Castling(0b11000000).can(WhiteKing));
        assert!(Castling(0b11000000).can(WhiteQueen));

        assert!(!Castling(0b11000000).can(BlackKing));
        assert!(!Castling(0b11000000).can(BlackQueen));
    }

    #[test]
    pub fn black_both() {
        assert!(Castling(0b00110000).can(BlackKing));
        assert!(Castling(0b00110000).can(BlackQueen));

        assert!(!Castling(0b00110000).can(WhiteKing));
        assert!(!Castling(0b00110000).can(WhiteQueen));
    }

    #[test]
    pub fn all() {
        assert!(Castling::initial().can(WhiteKing));
        assert!(Castling::initial().can(WhiteQueen));
        assert!(Castling::initial().can(BlackKing));
        assert!(Castling::initial().can(BlackQueen));
    }
}
