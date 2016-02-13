use std::fmt;
use piece::Piece;

const HEIGHT:usize = 6;
const WIDTH:usize = 7;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    columns: [[Option<Piece>; HEIGHT]; WIDTH],
    pub active_piece: Piece
}

impl Board {
    pub fn new() -> Board {
        Board {
            columns: [[None; HEIGHT]; WIDTH],
            active_piece: Piece::Red
        }
    }

    pub fn make_move(&self, column_index: usize) -> Board {
        let mut new_board = self.clone();
        let mut column = new_board.columns[column_index];
        for index in (0..HEIGHT).rev() {
            if column[index].is_none() {
                column[index] = Some(self.active_piece);
                new_board.columns[column_index] = column;
                new_board.active_piece = self.active_piece.opponent();
                break;
            }
        };
        new_board
    }

    pub fn get_winner(&self) -> Option<Piece> {
        let mut winner = None;
        let vertical_win = self.has_vertical_win();
        let horizontal_win = self.has_horizontal_win();
        if horizontal_win.is_some() {
            winner = horizontal_win;
        } else if vertical_win.is_some() {
            winner = vertical_win;
        }
        winner
    }

    fn has_vertical_win(&self) -> Option<Piece> {
        let mut winner = None;
        for player in &[Piece::Red, Piece::Black] {
            for column in &self.columns {
                for index in (HEIGHT-3)..HEIGHT {
                    if column[(index-3)..].iter().all(|&piece| piece.is_some() && piece.unwrap() == *player) {
                        winner = Some(*player)
                    }
                }
            }
        }
        winner
    }

    fn has_horizontal_win(&self) -> Option<Piece> {
        let mut winner = None;
        for player in &[Piece::Red, Piece::Black] {
            for starting_index in 0..(WIDTH-3) {
                for row in 0..(HEIGHT) {
                    let mut pieces = [None; 4];
                    let mut counter = 0;
                    for index in starting_index..(starting_index+3) {
                        pieces[counter] = self.columns[index][row];
                        counter += 1;
                    }
                    if pieces.iter().all(|&piece| piece.is_some() && piece.unwrap() == *player ) {
                        winner = Some(*player)
                    }
                }
            }
        }
        winner
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = String::new();
        for i in 0..HEIGHT {
            for column in &self.columns {
                if let Some(ref space) = column[i] {
                    board.push_str(&space.to_string());
                } else {
                    board.push_str(" ");
                }
                board.push_str("|");
            }
            board.push_str("\n");
        }
        write!(f, "{}", board)
    }
}
