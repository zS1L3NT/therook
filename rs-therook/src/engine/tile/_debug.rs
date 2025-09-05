use super::*;

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = u8::from(*self) as usize;

        write!(f, "{}{}", "abcdefgh".chars().nth(index & 7).unwrap(), (index >> 3) + 1)
    }
}
