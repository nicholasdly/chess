use std::fmt;

use super::{ Color, Piece };
use super::bitboards::Bitboards;

pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub enum FenError {
    InvalidFields { fields: usize },
    IncorrectRankCount { ranks: usize },
    UnrecognizedPiece { piece: char },
    InvalidFenString
}

impl fmt::Debug for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FenError::InvalidFields { fields } => {
                writeln!(f, "invalid number of fen fields: {fields}, expected 6")
            },
            FenError::IncorrectRankCount { ranks } => {
                writeln!(f, "incorrect number of ranks: {ranks}, expected 8")
            },
            FenError::UnrecognizedPiece { piece } => {
                writeln!(f, "unrecognized piece char: {piece}")
            },
            FenError::InvalidFenString => {
                writeln!(f, "invalid fen string")
            }
        }
    }
}

/// Parses a FEN string.
pub fn parse_fen(fen: &str) -> Result<(Bitboards, Color, u16, u16), FenError> {

        // Split FEN string into its six parts:
        // [ piece placement, active color, castling rights, en passant targets, halfmoves, fullmoves ]
        let split_fen: Vec<&str> = fen.split_whitespace().collect();
        if split_fen.len() != 6 {
            return Err(FenError::InvalidFields { fields: split_fen.len() });
        }

        // Validate board representation.
        let board_repr: Vec<&str> = split_fen[0].split('/').rev().collect();
        if board_repr.len() != 8 {
            return Err(FenError::InvalidFenString);
        }

        // Parse board representation to populate bitboards.
        let mut bitboards: Bitboards = [[0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]];
        let mut i: u8 = 0;
        for c in board_repr.join("").chars() {
            if c.is_alphabetic() {
                let color = if c.is_uppercase() { Color::White } else { Color::Black };
                let piece = match c {
                    'p' | 'P' => Piece::Pawn,
                    'n' | 'N' => Piece::Knight,
                    'b' | 'B' => Piece::Bishop,
                    'r' | 'R' => Piece::Rook,
                    'q' | 'Q' => Piece::Queen,
                    'k' | 'K' => Piece::King,
                    _ => return Err(FenError::UnrecognizedPiece { piece: c })
                };
                bitboards[color as usize][piece as usize] |= 1 << i;
                i += 1;
            } else if c.is_numeric() {
                i += c.to_digit(10).unwrap() as u8;
            }
        }

        Ok((bitboards, Color::White, 0, 0))

}
