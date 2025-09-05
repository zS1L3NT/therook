use super::{super::Move, *};

impl Board {
    pub fn make_move(&mut self, r#move: Move) {
        // let start_tile = r#move.get_start();
        // let end_tile = r#move.get_end();
        // let flag = r#move.get_flag();
        // let promotion_piece_type = r#move.get_promote_piece_type();
        //
        // let is_en_passant = flag == MoveFlag::EnPassant;
        //
        // let Some(piece) = self.squares[start_tile] else {
        //     panic!("empty start square")
        // };
        // let piece_type = piece.get_type();
        //
        // let captured = if is_en_passant {
        //     Some(Piece::new(piece.get_color().opposite(), PieceType::Pawn))
        // } else {
        //     self.squares[end_tile]
        // };
        // let captured_type = captured.map(|c| c.get_type());
        //
        // self.bitboards[piece] ^= start_tile | end_tile;
        // self.squares[start_tile] = None;
        // self.squares[end_tile] = Some(piece);
        //
        // if let Some(captured) = captured {
        //     let mut captured_tile = end_tile;
        //
        //     // For EnPassant, the captured tile and end tile is not the same
        //     if is_en_passant {
        //         let mut index: u8 = captured_tile.into();
        //
        //         match captured.get_color() {
        //             PieceColor::White => index += 8,
        //             PieceColor::Black => index -= 8,
        //         };
        //
        //         captured_tile = index.into();
        //     }
        //
        //     self.bitboards[captured] ^= Bitboard::from(captured_tile);
        // }
    }
}
