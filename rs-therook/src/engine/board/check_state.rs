use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CheckState {
    None,
    Single(u8),
    Double,
}
