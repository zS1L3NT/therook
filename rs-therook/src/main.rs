mod engine;

use engine::board::Board;

fn main() {
    let board = Board::initial();
    println!("{:?}", board);

    for r#move in board.calculate_moves() {
        println!("{:?}", r#move);
    }
}
