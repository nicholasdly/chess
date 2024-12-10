use core::fmt;

use crate::piece::Piece;

use super::board::Board;

pub static FEN_START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub static FEN_ENDGAME_POS: &str = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50";

pub enum FenError {
    NotEnoughFields { fields: usize },
    IncorrectRankCount { ranks: usize },
    IncorrectFileCount { rank: usize },
    UnrecognizedPiece { piece: char },
}

impl fmt::Debug for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FenError::NotEnoughFields { fields } => {
                writeln!(f, "invalid number of fen fields: {fields}, expected 6")
            }
            FenError::IncorrectRankCount { ranks } => {
                writeln!(f, "invalid number of ranks: {ranks}, expected 8")
            }
            FenError::IncorrectFileCount { rank } => {
                writeln!(f, "invalid number of files in rank {rank}, expected 8")
            }
            FenError::UnrecognizedPiece { piece } => {
                writeln!(f, "unrecognized piece: {piece}")
            }
        }
    }
}

pub fn parse_fen(fen: &str) -> Result<Board, FenError> {
    let fields: Vec<&str> = fen.split_whitespace().collect();

    // The FEN string must have exactly 6 fields.
    if fields.len() != 6 {
        return Err(FenError::NotEnoughFields {
            fields: fields.len(),
        });
    }

    let squares = parse_piece_placement(fields[0]).unwrap();

    let board = Board { squares };

    Ok(board)
}

fn parse_piece_placement(field: &str) -> Result<[Piece; 64], FenError> {
    let ranks: Vec<&str> = field.split('/').collect();

    // There must exist exactly 8 ranks.
    if ranks.len() != 8 {
        return Err(FenError::IncorrectRankCount { ranks: ranks.len() });
    }

    let mut squares: [Piece; 64] = [Piece { kind: 0, color: 0 }; 64];

    // Iterate over the ranks, parsing each character to its corresponding piece.
    for (i, rank) in ranks.iter().enumerate() {
        let mut index = i * 8;
        for c in rank.chars() {
            // Digits represent the number of consecutive empty squares.
            if c.is_digit(10) {
                index += c.to_digit(10).unwrap() as usize;
                continue;
            }

            let kind = match c {
                'p' | 'P' => 0b001,
                'n' | 'N' => 0b010,
                'b' | 'B' => 0b011,
                'r' | 'R' => 0b100,
                'q' | 'Q' => 0b101,
                'k' | 'K' => 0b110,
                _ => return Err(FenError::UnrecognizedPiece { piece: c }),
            };

            let color = match c.is_uppercase() {
                true => 0b0000,
                false => 0b1000,
            };

            squares[index] = Piece { kind, color };
            index += 1;
        }
    }

    Ok(squares)
}

fn parse_active_color(field: &str) -> Result<bool, FenError> {
    unimplemented!();
}

fn parse_castling_rights(field: &str) -> Result<bool, FenError> {
    unimplemented!();
}

fn parse_en_passant_target(field: &str) -> Result<bool, FenError> {
    unimplemented!();
}

fn parse_halfmoves(field: &str) -> Result<bool, FenError> {
    unimplemented!();
}

fn parse_fullmoves(field: &str) -> Result<bool, FenError> {
    unimplemented!();
}
