use super::*;
use std::io::{BufRead, BufReader, Read, Write};

impl Board<'_> {
    #[allow(dead_code)]
    fn perft_iter(&mut self, current: u8, target: u8) -> u64 {
        if current > target {
            return 1;
        }

        let mut count = 0;

        for r#move in self.calculate_moves() {
            self.make_move(r#move);
            count += self.perft_iter(current + 1, target);
            self.undo_move(r#move);
        }

        count
    }

    #[allow(dead_code)]
    fn perft(&mut self, depth: u8) -> u64 {
        assert!(depth > 0);

        let mut actual = 0;

        for r#move in self.calculate_moves() {
            self.make_move(r#move);
            let perft = self.perft_iter(1 + 1, depth);
            self.undo_move(r#move);

            println!("{move:?}: {perft}");
            actual += perft;
        }

        actual
    }

    #[allow(dead_code)]
    fn stockfish_perft_difference(&mut self, depth: u8) {
        let mut process = std::process::Command::new("../stockfish/stockfish")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = process.stdin.take().expect("Failed to open stdin");
        let stdout = process.stdout.take().expect("Failed to open stdin");
        let mut reader = BufReader::new(stdout);

        writeln!(stdin, "uci").expect("Failed to write to stdin");

        for line in reader.by_ref().lines() {
            if line.unwrap() == "uciok" {
                break;
            }
        }

        writeln!(stdin, "position fen {}", String::from(&*self)).expect("Failed to write to stdin");
        writeln!(stdin, "go perft {}", depth).expect("Failed to write to stdin");

        let regex = regex::Regex::new(r"(\w\d\w\d\w?): (\d+)").unwrap();
        let mut expected_perfts = vec![];
        let mut actual_perfts = vec![];

        for line in reader.by_ref().lines() {
            let line = line.unwrap();

            if let Some(captures) = regex.captures(&line) {
                expected_perfts.push((captures[1].to_owned(), captures[2].parse::<u64>().unwrap()));
                continue;
            }

            if line.contains("Nodes searched: ") {
                break;
            }
        }

        process.kill().expect("Failed to kill stockfish");

        for r#move in self.calculate_moves() {
            self.make_move(r#move);
            let perft = self.perft_iter(1 + 1, depth);
            self.undo_move(r#move);

            actual_perfts.push((format!("{move:?}"), perft));
        }

        expected_perfts.sort_by(|a, b| a.0.cmp(&b.0));
        actual_perfts.sort_by(|a, b| a.0.cmp(&b.0));

        let mut has_errors = false;
        let mut e_i = 0;
        let mut a_i = 0;
        while e_i < expected_perfts.len() && a_i < actual_perfts.len() {
            let (e_move, e_perft) = &expected_perfts[e_i];
            let (a_move, a_perft) = &actual_perfts[a_i];

            if e_move == a_move {
                if e_perft != a_perft {
                    println!(
                        "move: {e_move: <5}, expected: {e_perft}, actual: {a_perft}, difference: {}{}",
                        if a_perft > e_perft { '+' } else { '-' },
                        a_perft.abs_diff(*e_perft)
                    );
                    has_errors = true;
                }
                e_i += 1;
                a_i += 1;
            } else {
                if actual_perfts.iter().find(|p| p.0 == *e_move).is_none() {
                    println!("move: {e_move: <5}, missing move detected");
                    has_errors = true;
                    e_i += 1;
                }

                if expected_perfts.iter().find(|p| p.0 == *a_move).is_none() {
                    println!("move: {a_move: <5}, invalid move detected");
                    has_errors = true;
                    a_i += 1;
                }

                if !has_errors {
                    unreachable!();
                }
            }
        }

        assert!(
            !has_errors,
            "Errors found with perft comparisons to stockfish"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://www.chessprogramming.org/Perft_Results#Initial_Position
    #[test]
    fn perft_initial() {
        let computed = Computed::new();
        let board = Board::initial(&computed);

        perft_expect(&mut board.clone(), 1, 20);
        perft_expect(&mut board.clone(), 2, 400);
        perft_expect(&mut board.clone(), 3, 8_902);
        perft_expect(&mut board.clone(), 4, 197_281);
        perft_expect(&mut board.clone(), 5, 4_865_609);
        // perft_expect(&mut board.clone(), 6, 119_060_324);
        // perft_expect(&mut board.clone(), 7, 3_195_901_860);
        // perft_expect(&mut board.clone(), 8, 84_998_978_956);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_2
    #[test]
    fn perft_position_2() {
        let computed = Computed::new();
        let board = Board::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
            &computed,
        );

        perft_expect(&mut board.clone(), 1, 48);
        perft_expect(&mut board.clone(), 2, 2_039);
        perft_expect(&mut board.clone(), 3, 97_862);
        perft_expect(&mut board.clone(), 4, 4_085_603);
        // perft_expect(&mut board.clone(), 5, 193_690_690);
        // perft_expect(&mut board.clone(), 6, 8_031_647_685);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_3
    #[test]
    fn perft_position_3() {
        let computed = Computed::new();
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &computed);

        perft_expect(&mut board.clone(), 1, 14);
        perft_expect(&mut board.clone(), 2, 191);
        perft_expect(&mut board.clone(), 3, 2_812);
        perft_expect(&mut board.clone(), 4, 43_238);
        perft_expect(&mut board.clone(), 5, 674_624);
        perft_expect(&mut board.clone(), 6, 11_030_083);
        // perft_expect(&mut board.clone(), 7, 178_633_661);
        // perft_expect(&mut board.clone(), 8, 3_009_794_393);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_4
    #[test]
    fn perft_position_4() {
        let computed = Computed::new();
        let board = Board::from_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            &computed,
        );

        perft_expect(&mut board.clone(), 1, 6);
        perft_expect(&mut board.clone(), 2, 264);
        perft_expect(&mut board.clone(), 3, 9_467);
        perft_expect(&mut board.clone(), 4, 422_333);
        // perft_expect(&mut board.clone(), 5, 15_833_292);
        // perft_expect(&mut board.clone(), 6, 706_045_033);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_5
    #[test]
    fn perft_position_5() {
        let computed = Computed::new();
        let board = Board::from_fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            &computed,
        );

        perft_expect(&mut board.clone(), 1, 44);
        perft_expect(&mut board.clone(), 2, 1_486);
        perft_expect(&mut board.clone(), 3, 62_379);
        perft_expect(&mut board.clone(), 4, 2_103_487);
        // perft_expect(&mut board.clone(), 5, 89_941_194);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_6
    #[test]
    fn perft_position_6() {
        let computed = Computed::new();
        let board = Board::from_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            &computed,
        );

        perft_expect(&mut board.clone(), 1, 46);
        perft_expect(&mut board.clone(), 2, 2_079);
        perft_expect(&mut board.clone(), 3, 89_890);
        perft_expect(&mut board.clone(), 4, 3_894_594);
        // perft_expect(&mut board.clone(), 5, 164_075_551);
        // perft_expect(&mut board.clone(), 6, 6_923_051_137);
        // perft_expect(&mut board.clone(), 7, 287_188_994_746);
        // perft_expect(&mut board.clone(), 8, 11_923_589_843_526);
        // perft_expect(&mut board.clone(), 9, 490_154_852_788_714);
    }

    fn perft_expect(board: &mut Board, depth: u8, expected: u64) {
        let actual = board.perft(depth);

        assert_eq!(
            actual, expected,
            "Incorrect perft, expected: {expected}, actual: {actual}"
        );
    }
}
