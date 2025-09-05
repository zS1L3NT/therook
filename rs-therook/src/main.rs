mod engine;

use engine::*;
pub use therook::*;

fn main() {
    let mut board = Board::initial();
    println!("{board:?}");

    board.move_piece(tile!(E2), tile!(E4), WHITE_PAWN);
    board.move_piece(tile!(D7), tile!(D5), BLACK_PAWN);

    println!("{board:?}");

    for r#move in board.calculate_moves() {
        println!("{move:?}");
    }
}

#[macro_export]
macro_rules! timed {
    ($label:expr, $expr:expr) => {{
        let now = std::time::Instant::now();
        let result = { $expr };
        println!("{} in {} ns", $label, now.elapsed().as_nanos());
        result
    }};
}
