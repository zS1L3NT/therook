use super::*;

impl Board {
    #[allow(dead_code)]
    fn perft(&mut self, current: u8, target: u8) -> u32 {
        if current > target {
            return 1;
        }

        let mut count = 0;

        for r#move in self.calculate_moves() {
            self.make_move(r#move);
            count += self.perft(current + 1, target);
            self.undo_move(r#move);
        }

        count
    }

    #[allow(dead_code)]
    fn perft_expect(&mut self, depth: u8, expected: u32) {
        assert!(depth > 0);

        let mut actual = 0;

        for r#move in self.calculate_moves() {
            self.make_move(r#move);
            let perft = self.perft(1 + 1, depth);
            actual += perft;
            println!("{move:?}: {}", perft);
            self.undo_move(r#move);
        }

        assert_eq!(
            actual, expected,
            "Incorrect PERFT, expected: {expected}, actual: {actual}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_2() {
        let mut board = Board::initial();

        board.perft_expect(2, 400);
    }

    #[test]
    fn perft_3() {
        let mut board = Board::initial();

        board.perft_expect(3, 8902);
    }

    #[test]
    fn perft_4() {
        let mut board = Board::initial();

        board.perft_expect(4, 197281);
    }
}
