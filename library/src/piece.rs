use std::fmt;

pub enum PieceError {
    UnrecognizedPiece { piece: u8 },
    UnrecognizedColor { color: u8 },
}

impl fmt::Debug for PieceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PieceError::UnrecognizedPiece { piece } => {
                writeln!(f, "unrecognized piece: {piece}")
            }
            PieceError::UnrecognizedColor { color } => {
                writeln!(f, "unrecognized color: {color}")
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub kind: u8,
    pub color: u8,
}

impl Piece {
    pub fn to_string(&self) -> Result<String, PieceError> {
        let piece = match self.kind {
            0b000 => ".",
            0b001 => "p",
            0b010 => "n",
            0b011 => "b",
            0b100 => "r",
            0b101 => "q",
            0b110 => "k",
            _ => return Err(PieceError::UnrecognizedPiece { piece: self.kind }),
        };

        match self.color {
            0b0000 => return Ok(piece.to_uppercase()),
            0b1000 => return Ok(piece.to_lowercase()),
            _ => return Err(PieceError::UnrecognizedColor { color: self.color }),
        }
    }
}
