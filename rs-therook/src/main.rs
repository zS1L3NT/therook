pub mod engine;
pub mod interfaces;

use engine::*;
pub use therook::*;

fn main() {
    let computed = Computed::new();
    let board = Board::initial(&computed);
    println!("{board:?}");

    let moves = board.calculate_moves();
    println!("Moves[{}]: {moves:?}", moves.len());
}
