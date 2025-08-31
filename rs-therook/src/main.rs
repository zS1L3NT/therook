mod engine;

use engine::{Board, Move};
use therook::{bitboard, tile};

fn main() {
    let mut board = Board::initial();
    println!("{:?}", board);

    for r#move in board.calculate_moves() {
        println!("{:?}", r#move);
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    // let e4 = Move::new(
    //     *bitboard!(E2).get_indexes().first().unwrap(),
    //     *bitboard!(E4).get_indexes().first().unwrap(),
    //     Move::NO_FLAG,
    // );
    // board.make_move(e4);
    let x = bitboard!(A1);
    let x = tile!(A1);

    // println!("{:?}", board);
}
