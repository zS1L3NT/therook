use super::*;
use crate::interfaces::*;
use std::io::{Write, stdout};

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
    fn perft_compare_stockfish(&self, stockfish: &mut Stockfish, depth: u8) {
        let fen = String::from(&*self);

        stockfish.write(format!("position fen {fen}"));
        stockfish.write(format!("go perft {depth}"));

        let perft_regex = regex::Regex::new(r"(\w\d\w\d\w?): (\d+)").unwrap();
        let mut expected_perfts = vec![];
        let mut actual_perfts = vec![];

        for line in stockfish.read_until("Nodes searched:".into()) {
            if let Some(captures) = perft_regex.captures(&line) {
                expected_perfts.push((captures[1].to_owned(), captures[2].parse::<u64>().unwrap()));
                continue;
            }
        }

        let mut board = self.clone();
        for r#move in board.calculate_moves() {
            board.make_move(r#move);
            let perft = board.perft_iter(1 + 1, depth);
            board.undo_move(r#move);

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
                        "move: {e_move}, expected: {e_perft}, actual: {a_perft}, difference: {}{}",
                        if a_perft > e_perft { '+' } else { '-' },
                        a_perft.abs_diff(*e_perft)
                    );
                    stdout().flush().expect("Failed to flush stdout");
                    has_errors = true;
                }
                e_i += 1;
                a_i += 1;
            } else {
                if actual_perfts.iter().find(|p| p.0 == *e_move).is_none() {
                    println!("move: {e_move}, missing move detected");
                    stdout().flush().expect("Failed to flush stdout");
                    has_errors = true;
                    e_i += 1;
                }

                if expected_perfts.iter().find(|p| p.0 == *a_move).is_none() {
                    println!("move: {a_move}, invalid move detected");
                    stdout().flush().expect("Failed to flush stdout");
                    has_errors = true;
                    a_i += 1;
                }

                if !has_errors {
                    unreachable!();
                }
            }
        }

        if has_errors {
            assert!(
                !has_errors,
                "Errors found with perft comparisons to stockfish"
            );
        } else {
            println!("PERFT {depth}, FEN: {fen}");

            for (r#move, perft) in &expected_perfts {
                println!("{move}: {perft}");
            }

            println!(
                "Nodes searched: {}",
                &expected_perfts.iter().map(|p| p.1).sum::<u64>()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://www.chessprogramming.org/Perft_Results#Initial_Position
    #[test]
    fn perft_position_1() {
        let mut stockfish = Stockfish::new();
        let computed = Computed::new();
        let board = Board::initial(&computed);

        board.perft_compare_stockfish(&mut stockfish, 1); // 20
        board.perft_compare_stockfish(&mut stockfish, 2); // 400
        board.perft_compare_stockfish(&mut stockfish, 3); // 8_902
        board.perft_compare_stockfish(&mut stockfish, 4); // 197_281
        board.perft_compare_stockfish(&mut stockfish, 5); // 4_865_609
        // board.perft_compare_stockfish(&mut stockfish, 6); // 119_060_324
        // board.perft_compare_stockfish(&mut stockfish, 7); // 3_195_901_860
        // board.perft_compare_stockfish(&mut stockfish, 8); // 84_998_978_956
    }

    // https://www.chessprogramming.org/Perft_Results#Position_2
    #[test]
    fn perft_position_2() {
        let mut stockfish = Stockfish::new();
        let computed = Computed::new();
        let board = Board::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
            &computed,
        );

        board.perft_compare_stockfish(&mut stockfish, 1); // 48
        board.perft_compare_stockfish(&mut stockfish, 2); // 2_039
        board.perft_compare_stockfish(&mut stockfish, 3); // 97_862
        board.perft_compare_stockfish(&mut stockfish, 4); // 4_085_603
        // board.perft_compare_stockfish(&mut stockfish, 5); // 193_690_690
        // board.perft_compare_stockfish(&mut stockfish, 6); // 8_031_647_685
    }

    // https://www.chessprogramming.org/Perft_Results#Position_3
    #[test]
    fn perft_position_3() {
        let mut stockfish = Stockfish::new();
        let computed = Computed::new();
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &computed);

        board.perft_compare_stockfish(&mut stockfish, 1); // 14
        board.perft_compare_stockfish(&mut stockfish, 2); // 191
        board.perft_compare_stockfish(&mut stockfish, 3); // 2_812
        board.perft_compare_stockfish(&mut stockfish, 4); // 43_238
        board.perft_compare_stockfish(&mut stockfish, 5); // 674_624
        board.perft_compare_stockfish(&mut stockfish, 6); // 11_030_083
        // board.perft_compare_stockfish(&mut stockfish, 7); // 178_633_661
        // board.perft_compare_stockfish(&mut stockfish, 8); // 3_009_794_393
    }

    // https://www.chessprogramming.org/Perft_Results#Position_4
    #[test]
    fn perft_position_4() {
        let mut stockfish = Stockfish::new();
        let computed = Computed::new();
        let board = Board::from_fen(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            &computed,
        );

        board.perft_compare_stockfish(&mut stockfish, 1); // 6
        board.perft_compare_stockfish(&mut stockfish, 2); // 264
        board.perft_compare_stockfish(&mut stockfish, 3); // 9_467
        board.perft_compare_stockfish(&mut stockfish, 4); // 422_333
        board.perft_compare_stockfish(&mut stockfish, 5); // 15_833_292
        // board.perft_compare_stockfish(&mut stockfish, 6); // 706_045_033
    }

    // https://www.chessprogramming.org/Perft_Results#Position_5
    #[test]
    fn perft_position_5() {
        let mut stockfish = Stockfish::new();
        let computed = Computed::new();
        let board = Board::from_fen(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            &computed,
        );

        board.perft_compare_stockfish(&mut stockfish, 1); // 44
        board.perft_compare_stockfish(&mut stockfish, 2); // 1_486
        board.perft_compare_stockfish(&mut stockfish, 3); // 62_379
        board.perft_compare_stockfish(&mut stockfish, 4); // 2_103_487
        // board.perft_compare_stockfish(&mut stockfish, 5); // 89_941_194
    }

    // https://www.chessprogramming.org/Perft_Results#Position_6
    #[test]
    fn perft_position_6() {
        let mut stockfish = Stockfish::new();
        let computed = Computed::new();
        let board = Board::from_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            &computed,
        );

        board.perft_compare_stockfish(&mut stockfish, 1); // 46
        board.perft_compare_stockfish(&mut stockfish, 2); // 2_079
        board.perft_compare_stockfish(&mut stockfish, 3); // 89_890
        board.perft_compare_stockfish(&mut stockfish, 4); // 3_894_594
        // board.perft_compare_stockfish(&mut stockfish, 5); // 164_075_551
        // board.perft_compare_stockfish(&mut stockfish, 6); // 6_923_051_137
        // board.perft_compare_stockfish(&mut stockfish, 7); // 287_188_994_746
        // board.perft_compare_stockfish(&mut stockfish, 8); // 11_923_589_843_526
        // board.perft_compare_stockfish(&mut stockfish, 9); // 490_154_852_788_714
    }
}
