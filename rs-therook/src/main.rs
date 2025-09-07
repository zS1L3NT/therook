pub mod engine;
pub mod interfaces;

use engine::*;
pub use therook::*;

fn main() {
    // let mut board = Board::initial();
    let board = Board::fen("4k3/1p4pp/2p5/8/q3r2Q/3p3P/1P4PK/4R3 b - - 0 1".into()).unwrap();
    println!("{board:?}");

    // board.move_piece(tile!(E2), tile!(E4), WHITE_PAWN);
    // board.move_piece(tile!(D7), tile!(D5), BLACK_PAWN);
    // println!("{board:?}");

    for r#move in board.calculate_moves() {
        if r#move.get_start() == tile!(E4) {
            println!("{move:?}");
        }
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
