use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    Red,
    Black
}

impl Piece {
    pub fn opponent(&self) -> Piece {
        match *self {
            Piece::Red => Piece::Black,
            Piece::Black => Piece::Red
        }
    }

    pub fn player_name(&self) -> &str {
        match *self {
            Piece::Red => "Red",
            Piece::Black => "Black"
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Piece::Red => "r",
            Piece::Black => "b"
        };
        write!(f, "{}", symbol)
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Piece) -> bool {
        self.to_string() == other.to_string()
    }
}
