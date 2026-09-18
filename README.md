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
- Measured playing strength of around ~1M nodes/sec single-threaded; fastchess at `10+0.1` scores +359 against Maia-1100, +315 against Maia-1500 and +179 against Maia-1900, putting it on par with Stockfish capped at UCI_Elo 1540

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
