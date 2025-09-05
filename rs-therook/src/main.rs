mod engine;

use engine::*;
pub use therook::*;

fn main() {
    let mut board = Board::initial();
    println!("{:?}", board);

    for r#move in board.calculate_moves() {
        println!("{:?}", r#move);
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    board.make_move(Move::new(tile!(E2), tile!(E7), MoveFlag::None));

    println!("{:?}", board);
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
