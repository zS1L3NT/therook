use super::*;

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
            actual += perft;
            self.undo_move(r#move);

            println!("{move:?}: {perft}");
        }

        actual
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://www.chessprogramming.org/Perft_Results#Initial_Position
    #[test]
    fn perft_initial() {
        let computed = Computed::new();

        perft_expect(&mut Board::initial(&computed), 1, 20);
        perft_expect(&mut Board::initial(&computed), 2, 400);
        perft_expect(&mut Board::initial(&computed), 3, 8_902);
        perft_expect(&mut Board::initial(&computed), 4, 197_281);
        perft_expect(&mut Board::initial(&computed), 5, 4_865_609);
        // perft_expect(&mut Board::initial(), 6, 119_060_324);
        // perft_expect(&mut Board::initial(), 7, 3_195_901_860);
        // perft_expect(&mut Board::initial(), 8, 84_998_978_956);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_2
    #[test]
    fn perft_position_2() {
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
        let computed = Computed::new();

        perft_expect(&mut Board::from_fen(fen, &computed), 1, 48);
        perft_expect(&mut Board::from_fen(fen, &computed), 2, 2_039);
        perft_expect(&mut Board::from_fen(fen, &computed), 3, 97_862);
        perft_expect(&mut Board::from_fen(fen, &computed), 4, 4_085_603);
        // perft_expect(&mut Board::from_fen(fen, &computed), 5, 193_690_690);
        // perft_expect(&mut Board::from_fen(fen, &computed), 6, 8_031_647_685);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_3
    #[test]
    fn perft_position_3() {
        let fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
        let computed = Computed::new();

        perft_expect(&mut Board::from_fen(fen, &computed), 1, 14);
        perft_expect(&mut Board::from_fen(fen, &computed), 2, 191);
        perft_expect(&mut Board::from_fen(fen, &computed), 3, 2_812);
        perft_expect(&mut Board::from_fen(fen, &computed), 4, 43_238);
        perft_expect(&mut Board::from_fen(fen, &computed), 5, 674_624);
        // perft_expect(&mut Board::from_fen(fen, &computed), 6, 11_030_083);
        // perft_expect(&mut Board::from_fen(fen, &computed), 7, 178_633_661);
        // perft_expect(&mut Board::from_fen(fen, &computed), 8, 3_009_794_393);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_4
    #[test]
    fn perft_position_4() {
        let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
        let computed = Computed::new();

        perft_expect(&mut Board::from_fen(fen, &computed), 1, 6);
        perft_expect(&mut Board::from_fen(fen, &computed), 2, 264);
        perft_expect(&mut Board::from_fen(fen, &computed), 3, 9_467);
        perft_expect(&mut Board::from_fen(fen, &computed), 4, 422_333);
        // perft_expect(&mut Board::from_fen(fen, &computed), 5, 15_833_292);
        // perft_expect(&mut Board::from_fen(fen, &computed), 6, 706_045_033);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_5
    #[test]
    fn perft_position_5() {
        let fen = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        let computed = Computed::new();

        perft_expect(&mut Board::from_fen(fen, &computed), 1, 44);
        perft_expect(&mut Board::from_fen(fen, &computed), 2, 1_486);
        perft_expect(&mut Board::from_fen(fen, &computed), 3, 62_379);
        perft_expect(&mut Board::from_fen(fen, &computed), 4, 2_103_487);
        // perft_expect(&mut Board::from_fen(fen, &computed), 5, 89_941_194);
    }

    // https://www.chessprogramming.org/Perft_Results#Position_6
    #[test]
    fn perft_position_6() {
        let fen = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
        let computed = Computed::new();

        perft_expect(&mut Board::from_fen(fen, &computed), 1, 46);
        perft_expect(&mut Board::from_fen(fen, &computed), 2, 2_079);
        perft_expect(&mut Board::from_fen(fen, &computed), 3, 89_890);
        perft_expect(&mut Board::from_fen(fen, &computed), 4, 3_894_594);
        // perft_expect(&mut Board::from_fen(fen, &computed), 5, 164_075_551);
        // perft_expect(&mut Board::from_fen(fen, &computed), 6, 6_923_051_137);
        // perft_expect(&mut Board::from_fen(fen, &computed), 7, 287_188_994_746);
        // perft_expect(&mut Board::from_fen(fen, &computed), 8, 11_923_589_843_526);
        // perft_expect(&mut Board::from_fen(fen, &computed), 9, 490_154_852_788_714);
    }

    fn perft_expect(board: &mut Board, depth: u8, expected: u64) {
        let actual = board.perft(depth);

        assert_eq!(
            actual, expected,
            "Incorrect perft, expected: {expected}, actual: {actual}"
        );
    }
}
