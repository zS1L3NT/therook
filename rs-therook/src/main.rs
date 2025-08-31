mod engine;

use engine::{Board, Move, MoveFlag};
use therook::tile;

fn main() {
    let mut board = Board::initial();
    println!("{:?}", board);

    for r#move in board.calculate_moves() {
        println!("{:?}", r#move);
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    board.make_move(Move::new(tile!(E2), tile!(E4), MoveFlag::PawnDash));

    println!("{:?}", board);
}
