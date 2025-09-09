pub mod engine;
pub mod interfaces;

use engine::*;
pub use therook::*;

fn main() {
    let board = Board::initial();
    println!("{board:?}");

    let moves = board.calculate_moves();
    println!("Moves[{}]: {moves:?}", moves.len());
}

#[macro_export]
macro_rules! timed {
    ($label:expr, $expr:expr) => {{
        let now = std::time::Instant::now();
        let result = (|| $expr)();
        println!("{} in {} ns", $label, now.elapsed().as_nanos());
        result
    }};
}
