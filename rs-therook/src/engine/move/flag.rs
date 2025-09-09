use MoveFlag::*;

const NONE: u8 = 0;
const EN_PASSANT: u8 = 1;
const CASTLE: u8 = 2;
const PAWN_DASH: u8 = 3;
const PROMOTE_QUEEN: u8 = 4;
const PROMOTE_ROOK: u8 = 5;
const PROMOTE_BISHOP: u8 = 6;
const PROMOTE_KNIGHT: u8 = 7;

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum MoveFlag {
    None = NONE,
    EnPassant = EN_PASSANT,
    Castle = CASTLE,
    PawnDash = PAWN_DASH,
    PromoteQueen = PROMOTE_QUEEN,
    PromoteRook = PROMOTE_ROOK,
    PromoteBishop = PROMOTE_BISHOP,
    PromoteKnight = PROMOTE_KNIGHT,
}

impl From<MoveFlag> for u8 {
    fn from(flag: MoveFlag) -> Self {
        flag as u8
    }
}

impl From<u8> for MoveFlag {
    fn from(u8: u8) -> Self {
        match u8 {
            NONE => None,
            EN_PASSANT => EnPassant,
            CASTLE => Castle,
            PAWN_DASH => PawnDash,
            PROMOTE_QUEEN => PromoteQueen,
            PROMOTE_ROOK => PromoteRook,
            PROMOTE_BISHOP => PromoteBishop,
            PROMOTE_KNIGHT => PromoteKnight,
            _ => panic!("Unknown move type: {u8:?}"),
        }
    }
}
