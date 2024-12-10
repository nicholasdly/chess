use crate::{fen, piece::Piece};
use std::fmt;

/// Represents a chessboard.
pub struct Board {
    pub(super) squares: [Piece; 64],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Board {
    /// Creates a new `Board` in the starting position.
    pub fn new() -> Self {
        return Self::from_fen(fen::FEN_START_POS);
    }

    /// Parses a FEN string, returning the corresponding `Board`.
    pub fn from_fen(fen: &str) -> Self {
        return fen::parse_fen(fen).unwrap();
    }

    /// Returns a human-readable `String` representation of the `Board`.
    pub fn to_string(&self) -> String {
        let mut board = String::from("");

        for (index, piece) in self.squares.iter().enumerate() {
            if index > 0 && index % 8 == 0 {
                board += "\n";
            }

            board += &piece.to_string().unwrap();
        }

        return board;
    }
}
