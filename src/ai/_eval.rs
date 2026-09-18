use super::*;

pub fn material_value(piece_type: PieceType) -> i32 {
    match piece_type {
        PieceType::Pawn => 100,
        PieceType::Knight => 320,
        PieceType::Bishop => 330,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 0,
    }
}

/// Distance of a file (0-7) from the centre files d/e. 0 centre, 3 rim.
fn file_centre_distance(file: u8) -> i32 {
    let file = file as i32;
    (file - 3).abs().min((file - 4).abs())
}

/// Same for a rank already expressed from White's perspective (0 = rank 1).
fn rank_centre_distance(rank: u8) -> i32 {
    let rank = rank as i32;
    (rank - 3).abs().min((rank - 4).abs())
}

/// Hand-written middlegame square bonus, White's perspective.
/// Positive rewards, small on purpose so material still dominates.
fn mg_square_bonus(piece_type: PieceType, file: u8, rank: u8) -> i32 {
    let cd = file_centre_distance(file);
    let rd = rank_centre_distance(rank);
    match piece_type {
        PieceType::Pawn => rank as i32 * 10 + (3 - cd) * 2,
        PieceType::Knight => (6 - (cd + rd)) * 5 - 10,
        PieceType::Bishop => (6 - (cd + rd)) * 4 - 4,
        PieceType::Rook => {
            let seventh = if rank == 6 { 20 } else { 0 };
            seventh + (3 - cd) * 2
        }
        PieceType::Queen => (6 - (cd + rd)) * 2,
        PieceType::King => {
            // Kings hide on the back rank near the corners in the middlegame.
            let back_rank = if rank <= 1 {
                10 + cd * 4
            } else {
                -(rank as i32) * 5
            };
            -(cd + rd) * 6 + back_rank
        }
    }
}

/// Hand-written endgame square bonus. Kings centralise, pawns run.
fn eg_square_bonus(piece_type: PieceType, file: u8, rank: u8) -> i32 {
    let cd = file_centre_distance(file);
    let rd = rank_centre_distance(rank);
    match piece_type {
        PieceType::Pawn => rank as i32 * 15 + (3 - cd) * 2,
        PieceType::Knight => (6 - (cd + rd)) * 5 - 6,
        PieceType::Bishop => (6 - (cd + rd)) * 4,
        PieceType::Rook => {
            let seventh = if rank == 6 { 15 } else { 0 };
            seventh + (3 - cd) * 2
        }
        PieceType::Queen => (6 - (cd + rd)) * 3,
        PieceType::King => (6 - (cd + rd)) * 8,
    }
}

/// 24 = opening, 0 = bare endgame. Standard N/B=1, R=2, Q=4 weighting.
fn game_phase(board: &Board<'_>) -> i32 {
    let mut phase = 0;
    for color in [PieceColor::White, PieceColor::Black] {
        phase += u64::from(board.pieces[color | PieceType::Knight]).count_ones() as i32;
        phase += u64::from(board.pieces[color | PieceType::Bishop]).count_ones() as i32;
        phase += u64::from(board.pieces[color | PieceType::Rook]).count_ones() as i32 * 2;
        phase += u64::from(board.pieces[color | PieceType::Queen]).count_ones() as i32 * 4;
    }
    phase.min(24)
}

fn pawn_file_counts(board: &Board<'_>, color: PieceColor) -> [i32; 8] {
    let mut counts = [0; 8];
    for square in board.pieces[color | PieceType::Pawn] {
        counts[(square & 7) as usize] += 1;
    }
    counts
}

/// Small hand-tuned structure terms, White-relative. All values chosen by
/// feel and validated with fast-chess self-play, not copied from elsewhere.
fn structure_bonus(board: &Board<'_>) -> i32 {
    let mut score = 0;
    for color in [PieceColor::White, PieceColor::Black] {
        let sign = if color == PieceColor::White { 1 } else { -1 };
        let counts = pawn_file_counts(board, color);

        // Bishop pair.
        if u64::from(board.pieces[color | PieceType::Bishop]).count_ones() >= 2 {
            score += sign * 30;
        }

        // Doubled + isolated pawns.
        for file in 0..8 {
            if counts[file] > 1 {
                score -= sign * 12 * (counts[file] - 1);
            }
            if counts[file] > 0 {
                let left = if file > 0 { counts[file - 1] } else { 0 };
                let right = if file < 7 { counts[file + 1] } else { 0 };
                if left == 0 && right == 0 {
                    score -= sign * 10 * counts[file];
                }
            }
        }

        // Rook on open / semi-open file, and passed pawns.
        let enemy = color.opposite();
        let enemy_counts = pawn_file_counts(board, enemy);
        for square in board.pieces[color | PieceType::Rook] {
            let file = (square & 7) as usize;
            if counts[file] == 0 {
                score += sign * if enemy_counts[file] == 0 { 15 } else { 8 };
            }
        }
        for square in board.pieces[color | PieceType::Pawn] {
            let file = (square & 7) as usize;
            let rank = square >> 3;
            let advance = if color == PieceColor::White {
                rank as i32
            } else {
                7 - rank as i32
            };
            let blocked_ahead = (file.saturating_sub(1)..=(file + 1).min(7)).any(|f| {
                (0..8).any(|r| {
                    let ahead = if color == PieceColor::White {
                        r > rank
                    } else {
                        r < rank
                    };
                    ahead
                        && (u64::from(board.pieces[enemy | PieceType::Pawn]) >> (r * 8 + f as u8))
                            & 1
                            == 1
                })
            });
            if !blocked_ahead {
                score += sign * (8 + advance * 8);
            }
        }
    }
    score
}

pub fn evaluate(board: &Board<'_>) -> i32 {
    let mut mg = 0;
    let mut eg = 0;
    for piece_type in PieceType::ALL {
        let value = material_value(piece_type);
        for square in board.pieces[PieceColor::White | piece_type] {
            let file = square & 7;
            let rank = square >> 3;
            mg += value + mg_square_bonus(piece_type, file, rank);
            eg += value + eg_square_bonus(piece_type, file, rank);
        }
        for square in board.pieces[PieceColor::Black | piece_type] {
            // Mirror vertically so Black reuses White's tables.
            let flipped = square ^ 56;
            let file = flipped & 7;
            let rank = flipped >> 3;
            mg -= value + mg_square_bonus(piece_type, file, rank);
            eg -= value + eg_square_bonus(piece_type, file, rank);
        }
    }
    let phase = game_phase(board);
    let mut score = (mg * phase + eg * (24 - phase)) / 24;
    score += structure_bonus(board);
    // Tempo: side to move is worth a fraction of a pawn.
    score += if board.turn == PieceColor::White {
        10
    } else {
        -10
    };
    if board.turn == PieceColor::White {
        score
    } else {
        -score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluation_prefers_centre_and_material() {
        let computed = Computed::new();
        // Centralised knight outscores a rim knight, all else equal.
        let centre = Board::from_fen("4k3/8/8/8/3N4/8/8/4K3 w - - 0 1", &computed);
        let rim = Board::from_fen("4k3/8/8/8/8/8/8/N3K3 w - - 0 1", &computed);
        assert!(evaluate(&centre) > evaluate(&rim));

        // Extra queen is winning.
        let up_queen = Board::from_fen("4k3/8/8/8/8/8/8/3QK3 w - - 0 1", &computed);
        let lone = Board::from_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1", &computed);
        assert!(evaluate(&up_queen) > evaluate(&lone) + 500);
    }
}
