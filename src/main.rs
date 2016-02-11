mod piece;
mod board;
use board::Board;

fn main() {
    let board = Board::new();
    let board = board.make_move(0);
    let board = board.make_move(1);
    println!("{}", board.make_move(0));
}
