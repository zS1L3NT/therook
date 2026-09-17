use super::*;
use std::cmp::Reverse;

pub fn move_order_score(board: &Board<'_>, r#move: Move) -> i32 {
    // MVV-LVA: most valuable victim first, cheapest attacker breaks ties.
    // Victim dominates so PxQ outranks QxP; attacker term only orders equal victims.
    let attacker_value = board.squares[r#move.get_start() as usize]
        .map(|piece| material_value(piece.get_type()))
        .unwrap_or(0);
    let victim_value = if r#move.get_flag() == MoveFlag::EnPassant {
        material_value(PieceType::Pawn)
    } else {
        board.squares[r#move.get_end() as usize]
            .map(|piece| material_value(piece.get_type()))
            .unwrap_or(0)
    };
    let capture = if victim_value > 0 {
        victim_value * 10 - attacker_value / 10
    } else {
        0
    };
    let promotion = r#move
        .get_promote_piece_type()
        .map(|piece_type| material_value(piece_type) / 10)
        .unwrap_or(0);
    capture + promotion
}

/// Return the ordering of a move's UCI text without constructing that text.
///
/// UCI squares are written file-first (a1, a2, ..., h8), while the board's
/// square index is rank-first.  Reordering the two components here therefore
/// gives the same lexicographic ordering as `move_to_uci`.  Promotion suffixes
/// are ordered by their UCI characters: b, n, q, r.
pub fn uci_order_key(r#move: Move) -> u16 {
    let square_key = |square: u8| ((square & 7) as u16) * 8 + (square >> 3) as u16;
    let promotion_key = match r#move.get_promote_piece_type() {
        None => 0,
        Some(PieceType::Bishop) => 1,
        Some(PieceType::Knight) => 2,
        Some(PieceType::Queen) => 3,
        Some(PieceType::Rook) => 4,
        Some(PieceType::King | PieceType::Pawn) => unreachable!(),
    };
    (square_key(r#move.get_start()) * 64 + square_key(r#move.get_end())) * 5 + promotion_key
}

pub fn ordered_moves(board: &Board<'_>) -> Vec<Move> {
    let mut moves = board.calculate_moves();
    // Cache the two scalar keys once per move.  The previous comparator
    // recomputed both values for every comparison (and used to allocate UCI
    // strings for the tie-break), which is especially costly at leaf-heavy
    // depths.
    moves.sort_by_cached_key(|r#move| {
        (
            Reverse(move_order_score(board, *r#move)),
            uci_order_key(*r#move),
        )
    });
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_uci_key_matches_legacy_string_order() {
        let computed = Computed::new();
        for fen in [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "7k/P7/8/3pP3/8/8/8/7K w - - 0 1",
            "r3k2r/8/8/3pP3/8/8/8/R3K2R w KQkq d6 0 1",
        ] {
            let board = Board::from_fen(fen, &computed);
            let mut expected = board.calculate_moves();
            expected.sort_by(|a, b| {
                move_order_score(&board, *b)
                    .cmp(&move_order_score(&board, *a))
                    .then_with(|| move_to_uci(*a).cmp(&move_to_uci(*b)))
            });
            let actual = ordered_moves(&board);
            let expected_uci = expected.iter().map(|r#move| move_to_uci(*r#move));
            let actual_uci = actual.iter().map(|r#move| move_to_uci(*r#move));
            assert_eq!(
                actual_uci.collect::<Vec<_>>(),
                expected_uci.collect::<Vec<_>>()
            );
            assert_eq!(actual, ordered_moves(&board));
        }
    }
}
