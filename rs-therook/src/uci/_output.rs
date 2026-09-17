use super::*;

pub fn square_to_uci(square: u8) -> String {
    format!(
        "{}{}",
        (b'a' + (square & 7)) as char,
        (b'1' + (square >> 3)) as char
    )
}

pub fn move_to_uci(r#move: Move) -> String {
    let mut text = format!(
        "{}{}",
        square_to_uci(r#move.get_start()),
        square_to_uci(r#move.get_end())
    );
    if let Some(piece_type) = r#move.get_promote_piece_type() {
        text.push(match piece_type {
            PieceType::Queen => 'q',
            PieceType::Rook => 'r',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            _ => unreachable!(),
        });
    }
    text
}

pub fn score_to_uci(score: i32) -> String {
    if score >= MATE_THRESHOLD {
        let plies = MATE_SCORE - score;
        format!("mate {}", (plies + 1) / 2)
    } else if score <= -MATE_THRESHOLD {
        let plies = MATE_SCORE + score;
        format!("mate -{}", (plies + 1) / 2)
    } else {
        format!("cp {score}")
    }
}

pub fn write_line(output: &mut impl Write, line: &str) -> io::Result<()> {
    writeln!(output, "{line}")?;
    output.flush()
}
