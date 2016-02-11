mod piece;
mod board;
use board::Board;
use std::io;

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    loop {
        println!("Enter a column number to drop your piece");
        println!("{}'s turn:", board.player_name());
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        let column_number = input.trim().parse::<usize>();

        match column_number {
            Ok(x) => board = board.make_move(x - 1),
            Err(_) => println!("That is not a valid move")
        }
        println!("{}", board);
    }
}
