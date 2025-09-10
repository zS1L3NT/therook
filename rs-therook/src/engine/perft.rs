use super::*;

impl Board {
    fn perft(&mut self, depth: u8) -> u32 {
        if depth == 0 {
            return 1;
        }

        let mut count = 0;

        for r#move in self.calculate_moves() {
            self.make_move(r#move);
            count += self.perft(depth - 1);
            self.undo_move(r#move);
        }

        count
    }

    fn perft_expect(&mut self, depth: u8, expected: u32) {
        assert!(depth > 0);

        let mut actual = 0;

        for r#move in self.calculate_moves() {
            let perft = self.perft(depth - 1);
            println!("{move:?}: {}", perft);
            actual += perft;
        }

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut board = Board::initial();

        board.perft_expect(2, 400);
    }
}
