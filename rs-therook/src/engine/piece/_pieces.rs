use super::*;

pub const WHITE_KING: Piece = Piece((PieceColor::White as u8) << 5 | PieceType::King as u8);
pub const WHITE_QUEEN: Piece = Piece((PieceColor::White as u8) << 5 | PieceType::Queen as u8);
pub const WHITE_ROOK: Piece = Piece((PieceColor::White as u8) << 5 | PieceType::Rook as u8);
pub const WHITE_BISHOP: Piece = Piece((PieceColor::White as u8) << 5 | PieceType::Bishop as u8);
pub const WHITE_KNIGHT: Piece = Piece((PieceColor::White as u8) << 5 | PieceType::Knight as u8);
pub const WHITE_PAWN: Piece = Piece((PieceColor::White as u8) << 5 | PieceType::Pawn as u8);

pub const BLACK_KING: Piece = Piece((PieceColor::Black as u8) << 5 | PieceType::King as u8);
pub const BLACK_QUEEN: Piece = Piece((PieceColor::Black as u8) << 5 | PieceType::Queen as u8);
pub const BLACK_ROOK: Piece = Piece((PieceColor::Black as u8) << 5 | PieceType::Rook as u8);
pub const BLACK_BISHOP: Piece = Piece((PieceColor::Black as u8) << 5 | PieceType::Bishop as u8);
pub const BLACK_KNIGHT: Piece = Piece((PieceColor::Black as u8) << 5 | PieceType::Knight as u8);
pub const BLACK_PAWN: Piece = Piece((PieceColor::Black as u8) << 5 | PieceType::Pawn as u8);
