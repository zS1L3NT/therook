use super::*;
use std::panic::{AssertUnwindSafe, catch_unwind};

pub fn parse_square(text: &str) -> Option<u8> {
    let bytes = text.as_bytes();
    if bytes.len() != 2 || !(b'a'..=b'h').contains(&bytes[0]) || !(b'1'..=b'8').contains(&bytes[1])
    {
        return None;
    }
    Some((bytes[1] - b'1') * 8 + bytes[0] - b'a')
}

/// Match a move against the legal move list, retaining special-move flags.
pub fn parse_legal_move(board: &Board<'_>, text: &str) -> Option<Move> {
    let text = text.trim().to_ascii_lowercase();
    if text.len() != 4 && text.len() != 5 {
        return None;
    }
    let start = parse_square(&text[0..2])?;
    let end = parse_square(&text[2..4])?;
    let promotion = text.as_bytes().get(4).copied();
    if promotion.is_some() && !matches!(promotion, Some(b'q' | b'r' | b'b' | b'n')) {
        return None;
    }

    board.calculate_moves().into_iter().find(|candidate| {
        candidate.get_start() == start
            && candidate.get_end() == end
            && match (promotion, candidate.get_promote_piece_type()) {
                (None, None) => true,
                (Some(b'q'), Some(PieceType::Queen))
                | (Some(b'r'), Some(PieceType::Rook))
                | (Some(b'b'), Some(PieceType::Bishop))
                | (Some(b'n'), Some(PieceType::Knight)) => true,
                _ => false,
            }
    })
}

pub fn apply_uci_moves(board: &mut Board<'_>, moves: &[&str]) -> Result<(), String> {
    for move_text in moves {
        let Some(r#move) = parse_legal_move(board, move_text) else {
            return Err((*move_text).to_string());
        };
        board.make_move(r#move);
    }
    Ok(())
}

pub fn parse_position(board: &mut Board<'_>, tokens: &[&str]) -> Result<(), String> {
    if tokens.first() == Some(&"startpos") {
        let mut replacement = Board::initial(board.computed);
        let moves = match tokens {
            [_] => &[][..],
            [_, "moves", moves @ ..] => moves,
            _ => return Err("expected `startpos` or `startpos moves ...`".to_string()),
        };
        apply_uci_moves(&mut replacement, moves)?;
        *board = replacement;
        return Ok(());
    }
    if tokens.first() != Some(&"fen") || tokens.len() < 7 {
        return Err("expected startpos or six-field fen".to_string());
    }
    let moves_index = tokens.iter().position(|token| *token == "moves");
    let fen_end = moves_index.unwrap_or(tokens.len());
    if fen_end != 7 {
        return Err("expected exactly six fen fields".to_string());
    }
    let fen = tokens[1..7].join(" ");
    let computed = board.computed;
    // The legacy FEN reader finalizes the fullmove field on a trailing space.
    // Add one here because UCI's six fields are normally passed without it.
    let fen_with_separator = format!("{fen} ");
    let mut replacement = catch_unwind(AssertUnwindSafe(|| {
        Board::from_fen(&fen_with_separator, computed)
    }))
    .map_err(|_| "invalid fen".to_string())?;
    if let Some(index) = moves_index {
        apply_uci_moves(&mut replacement, &tokens[index + 1..])?;
    }
    *board = replacement;
    Ok(())
}

#[cfg(test)]
mod tests {
use super::*;
use std::panic::{AssertUnwindSafe, catch_unwind};

    #[test]
    fn uci_move_round_trip_restores_position() {
        let computed = Computed::new();
        let fen = Board::initial(&computed).to_fen();
        let mut board = Board::initial(&computed);
        let r#move = parse_legal_move(&board, "e2e4").unwrap();
        board.make_move(r#move);
        assert_eq!(move_to_uci(r#move), "e2e4");
        board.undo_move(r#move);
        assert_eq!(board.to_fen(), fen);
    }

    #[test]
    fn position_moves_are_applied_with_uci_flags() {
        let computed = Computed::new();
        let mut board = Board::initial(&computed);
        let tokens = "startpos moves e2e4 e7e5 g1f3"
            .split_whitespace()
            .collect::<Vec<_>>();
        parse_position(&mut board, &tokens).unwrap();
        assert_eq!(
            board.to_fen(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
        );
    }

    #[test]
    fn startpos_syntax_is_strict_and_atomic() {
        let computed = Computed::new();
        let mut board = Board::initial(&computed);
        let original = board.to_fen();
        let invalid = "startpos typo e2e4".split_whitespace().collect::<Vec<_>>();
        assert!(parse_position(&mut board, &invalid).is_err());
        assert_eq!(board.to_fen(), original);
    }
}
