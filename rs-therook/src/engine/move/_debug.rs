use super::*;

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            Self::format_square(self.get_start()),
            Self::format_square(self.get_end()),
            Self::format_promotion(self.get_promote_piece_type())
        )
    }
}

impl Move {
    fn format_square(square: u8) -> String {
        format!(
            "{}{}",
            ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][square as usize & 7],
            ['1', '2', '3', '4', '5', '6', '7', '8'][square as usize >> 3]
        )
    }

    fn format_promotion(promotion_piece_type: Option<PieceType>) -> String {
        if let Some(piece_type) = promotion_piece_type {
            match piece_type {
                PieceType::Queen => "q".into(),
                PieceType::Rook => "r".into(),
                PieceType::Bishop => "b".into(),
                PieceType::Knight => "n".into(),
                _ => unreachable!(),
            }
        } else {
            "".into()
        }
    }
}
