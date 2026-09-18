pub mod ai;
pub mod engine;
pub mod interfaces;
mod uci;

use ai::*;
use engine::*;
pub use therook::*;
use uci::*;

use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

const MATE_SCORE: i32 = 100_000;
const MATE_THRESHOLD: i32 = MATE_SCORE - 1_000;
const INFINITY: i32 = MATE_SCORE + 1_000;
const DEFAULT_DEPTH: u8 = 4;

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
        .unwrap_or_else(|| "0000".to_string());
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
