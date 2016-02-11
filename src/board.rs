use std::fmt;
use piece::Piece;

const HEIGHT:usize = 6;
const WIDTH:usize = 7;

#[derive(Debug, Copy, Clone)]
pub struct Board {
    columns: [[Option<Piece>; HEIGHT]; WIDTH],
    active_piece: Piece
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

    pub fn player_name(&self) -> &str {
        match self.active_piece {
            Piece::Red => "Red",
            Piece::Black => "Black"
        }
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
