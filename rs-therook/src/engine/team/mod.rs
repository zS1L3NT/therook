use Team::*;

pub enum Team {
    White,
    Black,
}

impl Team {
    pub fn code(&self) -> char {
        match self {
            White => 'w',
            Black => 'b',
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            White => Black,
            Black => White,
        }
    }
}
