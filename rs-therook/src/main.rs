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
