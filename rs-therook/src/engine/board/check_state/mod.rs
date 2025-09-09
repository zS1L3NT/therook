use super::*;

#[derive(PartialEq, Eq)]
pub enum CheckState {
    None,
    Single(Tile),
    Double,
}
