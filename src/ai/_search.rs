use super::*;

/// Extra PV rows for quiescence capture chains beyond nominal depth.
pub const QUIESCE_PLY_HEADROOM: usize = 64;

pub struct SearchContext {
    pub deadline: Option<Instant>,
    pub nodes: u64,
    pub cancel: Arc<AtomicBool>,
    pub pv: Vec<Vec<Move>>,
}

impl SearchContext {
    pub fn new(deadline: Option<Instant>, cancel: Arc<AtomicBool>, max_depth: u8) -> Self {
        // Quiescence extends past the nominal depth on capture chains,
        // so reserve headroom beyond the deepest nominal ply.
        let rows = usize::from(max_depth) + QUIESCE_PLY_HEADROOM + 1;
        Self {
            deadline,
            nodes: 0,
            cancel,
            pv: (0..rows).map(|_| Vec::with_capacity(rows)).collect(),
        }
    }

    pub fn expired(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
            || self
                .deadline
                .is_some_and(|deadline| Instant::now() >= deadline)
    }
}

pub struct SearchLine {
    pub score: i32,
    pub pv_len: usize,
}

pub fn set_pv(context: &mut SearchContext, ply: usize, r#move: Move, child_len: usize) {
    let (current_rows, child_rows) = context.pv.split_at_mut(ply + 1);
    let current = &mut current_rows[ply];
    let child = &child_rows[0];
    current.clear();
    current.push(r#move);
    current.extend_from_slice(&child[..child_len]);
}

fn is_capture_or_promotion(board: &Board<'_>, r#move: Move) -> bool {
    r#move.get_promote_piece_type().is_some()
        || r#move.get_flag() == MoveFlag::EnPassant
        || board.squares[r#move.get_end() as usize].is_some()
}

/// Captures-only extension so leaf evals are not taken mid-exchange.
/// In check, all evasions are searched so mates are never missed.
fn quiescence(
    board: &mut Board<'_>,
    mut alpha: i32,
    beta: i32,
    ply: i32,
    context: &mut SearchContext,
) -> Option<SearchLine> {
    if context.expired() {
        return None;
    }
    // Guard against pathological capture chains exhausting the PV rows.
    if ply as usize >= context.pv.len() {
        return Some(SearchLine {
            score: evaluate(board),
            pv_len: 0,
        });
    }
    context.nodes += 1;

    let moves = ordered_moves(board);
    if moves.is_empty() {
        let score = if board.check_state[board.turn] != CheckState::None {
            -MATE_SCORE + ply
        } else {
            0
        };
        return Some(SearchLine { score, pv_len: 0 });
    }

    let in_check = board.check_state[board.turn] != CheckState::None;
    if !in_check {
        let stand_pat = evaluate(board);
        if stand_pat >= beta {
            return Some(SearchLine {
                score: beta,
                pv_len: 0,
            });
        }
        alpha = alpha.max(stand_pat);
    }

    context.pv[ply as usize].clear();
    let mut best_score = if in_check { -INFINITY } else { alpha };
    let mut best_len = 0;
    let mut searched_any = false;
    for r#move in moves {
        if !in_check && !is_capture_or_promotion(board, r#move) {
            continue;
        }
        if context.expired() {
            return None;
        }
        searched_any = true;
        board.make_move(r#move);
        let child = quiescence(board, -beta, -alpha, ply + 1, context);
        board.undo_move(r#move);
        let child = child?;
        let score = -child.score;
        if score > best_score {
            set_pv(context, ply as usize, r#move, child.pv_len);
            best_score = score;
            best_len = child.pv_len + 1;
        }
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }
    if !searched_any {
        // Quiet position: stand pat already folded into alpha.
        return Some(SearchLine {
            score: alpha,
            pv_len: 0,
        });
    }
    Some(SearchLine {
        score: best_score,
        pv_len: best_len,
    })
}

pub fn negamax(
    board: &mut Board<'_>,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    ply: i32,
    context: &mut SearchContext,
) -> Option<SearchLine> {
    if context.expired() {
        return None;
    }
    context.nodes += 1;

    let moves = ordered_moves(board);
    if moves.is_empty() {
        let score = if board.check_state[board.turn] != CheckState::None {
            -MATE_SCORE + ply
        } else {
            0
        };
        return Some(SearchLine { score, pv_len: 0 });
    }
    if depth == 0 {
        return quiescence(board, alpha, beta, ply, context);
    }

    context.pv[ply as usize].clear();
    let mut best_score = -INFINITY;
    let mut best_len = 0;
    for r#move in moves {
        if context.expired() {
            return None;
        }
        board.make_move(r#move);
        let child = negamax(board, depth - 1, -beta, -alpha, ply + 1, context);
        board.undo_move(r#move);
        let child = child?;
        let score = -child.score;
        if score > best_score {
            set_pv(context, ply as usize, r#move, child.pv_len);
            best_score = score;
            best_len = child.pv_len + 1;
        }
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }
    Some(SearchLine {
        score: best_score,
        pv_len: best_len,
    })
}

fn search_root(
    board: &mut Board<'_>,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    context: &mut SearchContext,
) -> Option<SearchLine> {
    let moves = ordered_moves(board);
    context.pv[0].clear();
    let mut best_score = -INFINITY;
    let mut best_len = 0;
    for r#move in moves {
        if context.expired() {
            return None;
        }
        board.make_move(r#move);
        let child = negamax(board, depth.saturating_sub(1), -beta, -alpha, 1, context);
        board.undo_move(r#move);
        let child = child?;
        let score = -child.score;
        if score > best_score {
            set_pv(context, 0, r#move, child.pv_len);
            best_score = score;
            best_len = child.pv_len + 1;
        }
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }
    Some(SearchLine {
        score: best_score,
        pv_len: best_len,
    })
}

pub struct SearchInfo {
    pub depth: u8,
    pub score: i32,
    pub nodes: u64,
    pub elapsed_ms: u128,
    pub pv: Vec<Move>,
}

pub struct SearchReport {
    pub best: Option<Move>,
    pub infos: Vec<SearchInfo>,
    pub gameover: Option<&'static str>,
}

pub fn search(
    board: &mut Board<'_>,
    requested_depth: Option<u8>,
    movetime_ms: Option<u64>,
    infinite: bool,
    cancel: Arc<AtomicBool>,
) -> SearchReport {
    let started = Instant::now();
    let root_moves = ordered_moves(board);
    if root_moves.is_empty() {
        let gameover = if board.check_state[board.turn] != CheckState::None {
            "checkmate"
        } else {
            "stalemate"
        };
        return SearchReport {
            best: None,
            infos: vec![],
            gameover: Some(gameover),
        };
    }

    let deadline = movetime_ms.map(|ms| started + Duration::from_millis(ms));
    let max_depth = requested_depth.unwrap_or(64).max(1);
    let pv_depth = if infinite { u8::MAX } else { max_depth };
    let mut context = SearchContext::new(deadline, cancel, pv_depth);
    let mut best = root_moves.first().copied();
    let mut infos = vec![];

    let mut depth = 1u8;
    loop {
        let Some(line) = search_root(board, depth, -INFINITY, INFINITY, &mut context) else {
            break;
        };
        let pv = context.pv[0][..line.pv_len].to_vec();
        best = pv.first().copied().or(best);
        infos.push(SearchInfo {
            depth,
            score: line.score,
            nodes: context.nodes,
            elapsed_ms: started.elapsed().as_millis(),
            pv,
        });
        if line.score.abs() >= MATE_THRESHOLD || context.expired() {
            break;
        }
        if !infinite && depth >= max_depth {
            break;
        }
        // At the representation limit, keep searching the deepest supported
        // iteration until `stop` arrives instead of ending `go infinite`.
        depth = depth.saturating_add(1);
    }

    SearchReport {
        best,
        infos,
        gameover: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_a_mate_in_one() {
        let computed = Computed::new();
        let mut board = Board::from_fen("7k/5Q2/6K1/8/8/8/8/8 w - - 0 1", &computed);
        let report = search(
            &mut board,
            Some(2),
            None,
            false,
            Arc::new(AtomicBool::new(false)),
        );
        let best = report.best.map(move_to_uci).unwrap();
        assert!(
            board
                .calculate_moves()
                .iter()
                .any(|r#move| move_to_uci(*r#move) == best)
        );
        assert!(report.infos.iter().any(|info| info.score >= MATE_THRESHOLD));
    }

    #[test]
    fn search_is_deterministic_with_reused_pv_storage() {
        let computed = Computed::new();
        let run = || {
            let mut board = Board::initial(&computed);
            let report = search(
                &mut board,
                Some(3),
                None,
                false,
                Arc::new(AtomicBool::new(false)),
            );
            let infos = report
                .infos
                .iter()
                .map(|info| {
                    (
                        info.depth,
                        info.score,
                        info.nodes,
                        info.pv
                            .iter()
                            .map(|r#move| move_to_uci(*r#move))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>();
            (report.best.map(move_to_uci), infos)
        };

        assert_eq!(run(), run());
    }

    #[test]
    fn infinite_search_pv_storage_reaches_the_u8_depth_limit() {
        let cancel = Arc::new(AtomicBool::new(false));
        let mut context = SearchContext::new(None, cancel, u8::MAX);
        let parent = Move::new(square!(E2), square!(E4), MoveFlag::PawnDash);
        let child = Move::new(square!(E7), square!(E5), MoveFlag::PawnDash);
        context.pv[usize::from(u8::MAX)].push(child);

        set_pv(&mut context, usize::from(u8::MAX) - 1, parent, 1);

        assert_eq!(
            context.pv.len(),
            usize::from(u8::MAX) + QUIESCE_PLY_HEADROOM + 1
        );
        assert_eq!(context.pv[usize::from(u8::MAX) - 1], vec![parent, child]);
    }

    #[test]
    fn cancelled_search_returns_a_restored_legal_position() {
        let computed = Computed::new();
        let mut board = Board::initial(&computed);
        let original = board.to_fen();
        let cancel = Arc::new(AtomicBool::new(true));
        let report = search(&mut board, None, None, true, cancel);
        assert!(report.infos.is_empty());
        assert!(report.best.is_some());
        assert_eq!(board.to_fen(), original);
    }

    #[test]
    fn search_recomputes_double_check_without_panicking() {
        let computed = Computed::new();
        let mut board = Board::initial(&computed);
        let position = "startpos moves d2d4 d7d5 c2c4 e7e6 c4d5 d8d5 d1a4 b7b5 a4a5 d5d4 a5b5 b8d7 a2a3 c7c6 b5c6 a8b8 a1a2 f8a3 a2a3 d4b4 a3c3 a7a5 b1a3 a5a4 a3b1 a4a3 b1a3 b4a5 b2b4 a5b4 a3b1 b4b1 c3a3 b1b4 a3c3 b4a5 c1a3 b8b1 e1d2 b1f1 c6c8";
        let tokens = position.split_whitespace().collect::<Vec<_>>();
        parse_position(&mut board, &tokens).unwrap();

        let before = board.to_fen();
        let report = search(
            &mut board,
            Some(2),
            None,
            false,
            Arc::new(AtomicBool::new(false)),
        );

        assert!(report.best.is_some());
        assert_eq!(board.to_fen(), before);
    }
}
