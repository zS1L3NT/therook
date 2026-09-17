![Therook Cover Image](docs/banner.png)

# Therook

Therook is a revamped Rust chess engine built on bitboards, with a
deterministic alpha-beta search over a standard UCI interface.

## Build

```bash
cargo build --release --manifest-path rs-therook/Cargo.toml
```

The binary speaks plain UCI (`uci`, `isready`, `ucinewgame`, `position`,
`go depth|movetime|wtime|btime|infinite`, `stop`, `quit`) and reports
`info depth score nodes time pv` plus `bestmove`, using `bestmove 0000`
with an `info string gameover checkmate|stalemate` marker in terminal
positions.

## Test matches with standard tools

There is no custom arena in this repo. Use
[fast-chess](https://github.com/Disservin/fast-chess) or
[cutechess-cli](https://github.com/cutechess/cutechess):

```bash
fastchess -engine cmd=./rs-therook/target/release/therook name=therook \
  -engine cmd=../rs-tauri-chess/src-tauri/target/release/rs-tauri-chess name=legacy \
  -each tc=0.1+0.01 -rounds 50 -repeat -concurrency 2

fastchess --compliance ./rs-therook/target/release/therook
```

## Tests

```bash
cargo test --manifest-path rs-therook/Cargo.toml -- --skip perft_position
```

The `perft_position_*` tests need a local `stockfish` binary and are
skipped in normal development. `initial_position_counts_without_external_engine`
covers move generation offline.
