pub mod engine;
pub mod interfaces;
mod uci;

use engine::*;
pub use therook::*;
use uci::*;

use std::cmp::Reverse;
use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

const MATE_SCORE: i32 = 100_000;
const MATE_THRESHOLD: i32 = MATE_SCORE - 1_000;
const INFINITY: i32 = MATE_SCORE + 1_000;
const DEFAULT_DEPTH: u8 = 4;

fn material_value(piece_type: PieceType) -> i32 {
    match piece_type {
        PieceType::Pawn => 100,
        PieceType::Knight => 320,
        PieceType::Bishop => 330,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 0,
    }
}

fn evaluate(board: &Board<'_>) -> i32 {
    let mut score = 0;
    for piece_type in PieceType::ALL {
        let value = material_value(piece_type);
        let white = u64::from(board.pieces[PieceColor::White | piece_type]).count_ones() as i32;
        let black = u64::from(board.pieces[PieceColor::Black | piece_type]).count_ones() as i32;
        score += value * (white - black);
    }
    if board.turn == PieceColor::White {
        score
    } else {
        -score
    }
}

fn move_order_score(board: &Board<'_>, r#move: Move) -> i32 {
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
fn uci_order_key(r#move: Move) -> u16 {
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

fn ordered_moves(board: &Board<'_>) -> Vec<Move> {
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

struct SearchContext {
    deadline: Option<Instant>,
    nodes: u64,
    cancel: Arc<AtomicBool>,
    pv: Vec<Vec<Move>>,
}

impl SearchContext {
    fn new(deadline: Option<Instant>, cancel: Arc<AtomicBool>, max_depth: u8) -> Self {
        let rows = usize::from(max_depth) + 1;
        Self {
            deadline,
            nodes: 0,
            cancel,
            pv: (0..rows).map(|_| Vec::with_capacity(rows)).collect(),
        }
    }

    fn expired(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
            || self
                .deadline
                .is_some_and(|deadline| Instant::now() >= deadline)
    }
}

struct SearchLine {
    score: i32,
    pv_len: usize,
}

fn set_pv(context: &mut SearchContext, ply: usize, r#move: Move, child_len: usize) {
    let (current_rows, child_rows) = context.pv.split_at_mut(ply + 1);
    let current = &mut current_rows[ply];
    let child = &child_rows[0];
    current.clear();
    current.push(r#move);
    current.extend_from_slice(&child[..child_len]);
}

fn negamax(
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
        return Some(SearchLine {
            score: evaluate(board),
            pv_len: 0,
        });
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

struct SearchInfo {
    depth: u8,
    score: i32,
    nodes: u64,
    elapsed_ms: u128,
    pv: Vec<Move>,
}

struct SearchReport {
    best: Option<Move>,
    infos: Vec<SearchInfo>,
    gameover: Option<&'static str>,
}

fn search(
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

fn handle_go(
    output: &mut impl Write,
    board: &mut Board<'_>,
    tokens: &[&str],
    cancel: Arc<AtomicBool>,
) -> io::Result<()> {
    let limits = parse_go_limits(tokens);
    let movetime = clock_movetime(limits, board.turn);
    // A movetime search is iterative until its deadline; the default depth is
    // only for a bare `go`, where no other limit was supplied.
    let search_depth = limits
        .depth
        .or_else(|| (!limits.infinite && movetime.is_none()).then_some(DEFAULT_DEPTH));
    let report = search(board, search_depth, movetime, limits.infinite, cancel);
    if let Some(gameover) = report.gameover {
        write_line(output, &format!("info string gameover {gameover}"))?;
        return write_line(output, "bestmove 0000");
    }
    for info in &report.infos {
        let pv = info
            .pv
            .iter()
            .map(|r#move| move_to_uci(*r#move))
            .collect::<Vec<_>>()
            .join(" ");
        write_line(
            output,
            &format!(
                "info depth {} score {} nodes {} time {} pv {}",
                info.depth,
                score_to_uci(info.score),
                info.nodes,
                info.elapsed_ms,
                pv
            ),
        )?;
    }
    let bestmove = report
        .best
        .map(move_to_uci)
        .unwrap_or_else(|| "0000".into());
    write_line(output, &format!("bestmove {bestmove}"))
}

struct InputLine {
    sequence: u64,
    text: String,
}

fn main() {
    let computed = Computed::new();
    let mut board = Board::initial(&computed);
    let mut output = io::BufWriter::new(io::stdout());
    let (input_tx, input_rx) = mpsc::channel::<InputLine>();
    let cancel = Arc::new(AtomicBool::new(false));
    let interrupt_sequence = Arc::new(AtomicU64::new(0));
    let reader_cancel = Arc::clone(&cancel);
    let reader_interrupt_sequence = Arc::clone(&interrupt_sequence);

    // Reading stdin concurrently lets stop/quit interrupt a synchronous search.
    // Sequence numbers prevent a stop already queued after go from being
    // cleared when the main thread begins that search.
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut infinite_requested = false;
        for (sequence, line) in stdin.lock().lines().enumerate() {
            let Ok(text) = line else { break };
            let command = text.split_whitespace().next();
            if command == Some("go") {
                infinite_requested = text.split_whitespace().any(|token| token == "infinite");
            }
            if matches!(command, Some("stop" | "quit")) {
                reader_interrupt_sequence.store(sequence as u64, Ordering::Release);
                reader_cancel.store(true, Ordering::Release);
            }
            if input_tx
                .send(InputLine {
                    sequence: sequence as u64,
                    text,
                })
                .is_err()
            {
                break;
            }
        }
        // An unbounded search must also terminate cleanly when its input pipe
        // closes without an explicit stop/quit command.
        if infinite_requested {
            reader_interrupt_sequence.store(u64::MAX, Ordering::Release);
            reader_cancel.store(true, Ordering::Release);
        }
    });

    for input in input_rx {
        let tokens = input.text.split_whitespace().collect::<Vec<_>>();
        let Some(command) = tokens.first().copied() else {
            continue;
        };
        let result = match command {
            "uci" => write_line(&mut output, "id name therook")
                .and_then(|_| write_line(&mut output, "id author therook contributors"))
                .and_then(|_| write_line(&mut output, "uciok")),
            "isready" => write_line(&mut output, "readyok"),
            "ucinewgame" => {
                board = Board::initial(&computed);
                Ok(())
            }
            "position" => parse_position(&mut board, &tokens[1..]).map_or_else(
                |error| {
                    write_line(
                        &mut output,
                        &format!("info string invalid position {error}"),
                    )
                },
                |_| Ok(()),
            ),
            "go" => {
                if interrupt_sequence.load(Ordering::Acquire) <= input.sequence {
                    cancel.store(false, Ordering::Release);
                }
                handle_go(&mut output, &mut board, &tokens[1..], Arc::clone(&cancel))
            }
            "stop" => Ok(()),
            "quit" => break,
            _ => write_line(
                &mut output,
                &format!("info string unknown command {command}"),
            ),
        };
        if result.is_err() {
            break;
        }
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

    #[test]
    fn bitboard_evaluation_matches_square_scan() {
        fn square_scan(board: &Board<'_>) -> i32 {
            let score = board.squares.iter().flatten().fold(0, |score, piece| {
                let value = material_value(piece.get_type());
                if piece.get_color() == PieceColor::White {
                    score + value
                } else {
                    score - value
                }
            });
            if board.turn == PieceColor::White {
                score
            } else {
                -score
            }
        }

        let computed = Computed::new();
        for fen in [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/ppp2ppp/2n1b3/3qp3/3P4/2P1PN2/PP3PPP/R1BQKB1R b KQkq - 4 8",
            "7k/2Q5/6K1/8/8/8/8/8 w - - 0 1",
        ] {
            let board = Board::from_fen(fen, &computed);
            assert_eq!(evaluate(&board), square_scan(&board));
        }
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

        assert_eq!(context.pv.len(), usize::from(u8::MAX) + 1);
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
