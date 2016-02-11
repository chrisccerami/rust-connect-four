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
