use super::*;

impl Board {
    pub fn calculate_moves(self) -> Vec<Move> {
        let now = std::time::Instant::now();

        let moves: Vec<Move> = vec![];

        // Calculate

        println!(
            "calculated legal moves in {} nanoseconds",
            now.elapsed().as_nanos()
        );

        moves
    }
}
