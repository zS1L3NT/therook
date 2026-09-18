# TheRook

![License](https://img.shields.io/github/license/zS1L3NT/therook?style=for-the-badge) ![Languages](https://img.shields.io/github/languages/count/zS1L3NT/therook?style=for-the-badge) ![Top Language](https://img.shields.io/github/languages/top/zS1L3NT/therook?style=for-the-badge) ![Commit Activity](https://img.shields.io/github/commit-activity/y/zS1L3NT/therook?style=for-the-badge) ![Last commit](https://img.shields.io/github/last-commit/zS1L3NT/therook?style=for-the-badge)

This project is a ground-up rewrite of [ThePawn](https://github.com/zS1L3NT/thepawn)'s engine: a Rust chess engine with full legal move generation and a deterministic alpha-beta search, shipped as a headless UCI binary.

## Motivation

I have always loved building chess engines, but this time I was looking at building a chess engine that can optimise the benefits that rust gives like low level control. I also was fascinated by how we can speed up a chess engine by making it do many bitwise operations rather than array operations since they're much much faster.

I also have an old implementation of a chess engine using rust called [ThePawn](https://github.com/zS1L3NT/thepawn) but mostly using vectors and array searching. I wanted to do a side-by-side comparison to see how much more performant I can make a chess engine just by using bitwise operations over array operations.

## Features

- Bitboard board representation with precomputed attack, ray, between and x-ray tables
- Complete legal move generation (castling, en passant including pin edge cases, promotions, double-check evasions) verified by [perft tests](./src/engine/perft.rs) reaching 10B nodes.
- Deterministic iterative-deepening negamax search with alpha-beta pruning, captures-only quiescence and MVV-LVA move ordering
- Hand-written tapered evaluation (middlegame/endgame piece-square tables, pawn structure, bishop pair, rooks on open files, passed pawns, tempo)
- Plain UCI interface (`uci`, `isready`, `ucinewgame`, `position`, `go depth|movetime|wtime|btime|infinite`, `stop`, `quit`) reporting `info depth score nodes time pv` + `bestmove`, compatible with any UCI chess GUI or tournament harness such as fastchess

## Strength

Measured playing strength of around **~1M nodes/sec single-threaded** on a MacBook M1 Pro; fastchess at `10+0.1` scores +359 against Maia-1100, +315 against Maia-1500 and +179 against Maia-1900, putting it on par with Stockfish capped at **UCI_Elo 1540**

### Search performance

| Depth | Nodes (mean) | Nodes (median) | Nodes (range) | Time (mean) | Speed | Branching (mean/median) |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | 61 | 57 | 32–175 | <1 ms | n/a | — |
| 2 | 494 | 442 | 140–1,691 | 0.1 ms | ~0.96M nps | 8.4 / 7.5 |
| 3 | 4,516 | 4,021 | 1,094–13,551 | 4.5 ms | ~1.08M nps | 9.6 / 9.2 |
| 4 | 35,778 | 30,826 | 3,803–143,250 | 41 ms | ~0.89M nps | 7.7 / 7.3 |
| 5 | 293,040 | 253,569 | 31,867–1,487,740 | 336 ms | ~0.90M nps | 8.4 / 8.0 |

Single-threaded speed sits at around 1M nodes/sec with an effective branching factor of about 8. For reference, depth 6 from the start position searches 686,748 nodes in 698 ms, a middlegame depth 5 reaches 743k nodes in about a second, and a mate-in-1 is found after just 93 nodes.

### Playing strength vs Maia

Maia running under lc0 at one node per move (human-like instant play):

| Opponent | Games | Score (W-L-D) | Points | Elo diff |
| --- | --- | --- | --- | --- |
| maia-1100 | 40 | 31-0-9 | 88.75% | +359 ±103 |
| maia-1500 | 100 | 76-4-20 | 86.0% | +315 ±87 |
| maia-1900 | 40 | 25-6-9 | 73.75% | +179 ±106 |

### Playing strength vs Stockfish

Stockfish 19 with capped `UCI_Elo`, same time control and openings:

| Opponent | Games | Score (W-L-D) | Points | Elo diff |
| --- | --- | --- | --- | --- |
| SF Elo 1400 | 40 | 35-1-4 | 92.5% | +436 ±209 |
| SF Elo 1500 | 40 | 22-17-1 | 56.25% | +44 ±86 |
| SF Elo 1600 | 40 | 14-22-4 | 40.0% | -70 ±118 |
| SF Elo 1800 | 40 | 11-28-1 | 28.75% | -158 ±113 |

The 50% point lands at **Stockfish UCI_Elo ~1540** (full-strength Stockfish is out of reach at 0-30). Bridging the two scales puts Maia-1900 at ~1360, Maia-1500 at ~1225 and Maia-1100 at ~1180 Stockfish-Elo: instant-move Maia plays well below its human-rating label.

## Credits

I validated move generation against the perft results on the [Chess Programming Wiki](https://www.chessprogramming.org/Perft_Results), the same reference I used for thepawn. The project itself is the bitboard rewrite of [thepawn](https://github.com/zS1L3NT/thepawn), which was inspired by how Sebastian Lague built his engine in [this](https://www.youtube.com/watch?v=U4ogK0MIzqk) video. Playing strength was measured with [Maia](https://github.com/CSSLab/maia-chess), [fastchess](https://github.com/Disservin/fastchess) and [Stockfish](https://stockfishchess.org/).

## Usage

Build the engine with

```
$ cargo build --release
```

then run the binary and speak UCI to it

```
$ ./target/release/therook
uci
isready
position startpos
go depth 5
```

`go` also accepts `movetime`, clock-based `wtime`/`btime`/`winc`/`binc`, and `infinite` (paired with `stop`), while a bare `go` defaults to depth 4. `position` accepts `startpos` followed by moves or a full `fen` string.

## Tests

All 68 tests cover perft counts on the 6 standard positions, move generation edge cases (castling rights, en passant pins, promotions, double check), make/undo round-trips, evaluation/ordering/search determinism, and UCI position/limit parsing. Run them with

```
$ cargo test
```

The deeper perft positions take a while to run, so grab a coffee while waiting.
