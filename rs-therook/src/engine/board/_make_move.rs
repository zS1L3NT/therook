use super::*;

impl Board {
    pub fn make_move(&mut self, r#move: Move) {
        let start_square = r#move.get_start();
        let end_square = r#move.get_end();
        let flag = r#move.get_flag();
        let promotion_piece_type = r#move.get_promote_piece_type();

        let is_en_passant = flag == MoveFlag::EnPassant;

        let piece = self.squares[start_square as usize].unwrap();
        let color = piece.get_color();
        let opponent = color.opposite();

        let captured = if is_en_passant {
            Some(color.opposite() | PieceType::Pawn)
        } else {
            self.squares[end_square as usize]
        };
        let piece_type = piece.get_type();
        let captured_type = captured.map(|c| c.get_type());

        self.move_piece(start_square, end_square, piece);

        if let Some(captured) = captured {
            self.clear_square(
                if is_en_passant {
                    (start_square & 56) + (end_square & 7)
                } else {
                    end_square
                },
                captured,
            );
        }

        if piece_type.is_slider() {
            self.update_rays(color);
        }

        for color in [PieceColor::White, PieceColor::Black] {
            self.update_attacks(color);
            self.update_pin_lines(color);
        }
    }
}
