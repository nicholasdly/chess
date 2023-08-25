use std::fmt;

use super::{ Color, Piece };
use super::bitboards::Bitboards;

pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub enum FenError {
    InvalidFields { fields: usize },
    IncorrectRankCount { ranks: usize },
    UnrecognizedPiece { piece: char },
    UnrecognizedSquare { square: char },
    IncorrectSquareCount { count: u8 },
    UnrecognizedActiveColor { color: String },
    InvalidMoveField { moves: String },
    InvalidFenString
}

impl fmt::Debug for FenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FenError::InvalidFields { fields } => {
                writeln!(f, "invalid number of fen fields: {fields}, expected 6")
            },
            FenError::IncorrectRankCount { ranks } => {
                writeln!(f, "incorrect number of ranks: {ranks}, expected 8")
            },
            FenError::UnrecognizedPiece { piece } => {
                writeln!(f, "unrecognized piece char: {piece}")
            },
            FenError::UnrecognizedSquare { square } => {
                writeln!(f, "unrecognized square char: {square}, expected a valid piece char or integer in range 1..=8")
            },
            FenError::IncorrectSquareCount { count } => {
                writeln!(f, "incorrect number of squares: {count}, expected 64")
            }
            FenError::UnrecognizedActiveColor { color } => {
                writeln!(f, "unrecognized active color: {color}, expected 'w' or 'b'")
            },
            FenError::InvalidMoveField { moves } => {
                writeln!(f, "invalid move field: {moves}, expected unsigned 16-bit integer")
            },
            FenError::InvalidFenString => {
                writeln!(f, "invalid fen string")
            },
        }
    }
}

/// Parses a FEN string, returning a tuple of chess data on the current position.
/// 
/// # Examples
/// 
/// ```
/// use chess::board::{ fen, Color, Piece };
/// 
/// let fen = "8/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// let ( bitboards, active_color, halfmoves, fullmoves ) = fen::parse_fen(fen).unwrap();
/// 
/// assert_eq!(active_color, Color::White);
/// assert_eq!(halfmoves, 0);
/// assert_eq!(fullmoves, 1);
/// 
/// assert_eq!(bitboards[Color::White as usize][Piece::Pawn as usize], 0xFF00);
/// assert_eq!(bitboards[Color::White as usize][Piece::Knight as usize], 0x42);
/// assert_eq!(bitboards[Color::White as usize][Piece::Bishop as usize], 0x24);
/// assert_eq!(bitboards[Color::White as usize][Piece::Rook as usize], 0x81);
/// assert_eq!(bitboards[Color::White as usize][Piece::Queen as usize], 0x8);
/// assert_eq!(bitboards[Color::White as usize][Piece::King as usize], 0x10);
/// ```
/// 
/// # Panics
/// 
/// In the event the given FEN string is invalid or unrecognizable, a `FenError` is returned.
/// 
/// ```should_panic
/// use chess::board::{ fen, Color, Piece };
/// 
/// let fen = "obviously not a FEN string";
/// let ( bitboards, active_color, halfmoves, fullmoves ) = fen::parse_fen(fen).unwrap();
/// ```
pub fn parse_fen(fen: &str) -> Result<(Bitboards, Color, u16, u16), FenError> {

        // [ piece placement, active color, castling rights, en passant targets, halfmoves, fullmoves ]
        let fen: Vec<&str> = fen.split_whitespace().collect();
        if fen.len() != 6 {
            return Err(FenError::InvalidFields { fields: fen.len() });
        }

        let bitboards = parse_piece_placement(fen[0]).unwrap();
        let active_color = parse_active_color(fen[1]).unwrap();
        // TODO: Parse and return castling rights
        // TODO: Parse and return en passant targets
        let halfmoves = parse_move_count(fen[4]).unwrap();
        let fullmoves = parse_move_count(fen[5]).unwrap();

        Ok((bitboards, active_color, halfmoves, fullmoves))

}

/// Parse FEN piece placement string, generating each piece's respective bitboard.
fn parse_piece_placement(piece_placement: &str) -> Result<Bitboards, FenError> {

    let ranks: Vec<&str> = piece_placement.split('/').rev().collect();
    if ranks.len() != 8 {
        return Err(FenError::IncorrectRankCount { ranks: ranks.len() });
    }

    let mut bitboards: Bitboards = [[0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]];
    let mut i: u8 = 0;

    for c in ranks.join("").chars() {
        if c.is_alphabetic() {
            let color = match c.is_uppercase() {
                true => Color::White,
                false => Color::Black
            };
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
        } else if c.is_digit(10) {
            i += c.to_digit(10).unwrap() as u8;
        } else {
            return Err(FenError::UnrecognizedSquare { square: c })
        }
    }

    match i == 64 {
        true => Ok(bitboards),
        false => Err(FenError::IncorrectSquareCount { count: i })
    }
}

/// Parse FEN active color string, returning the current player's respective `Color`.
fn parse_active_color(active_color: &str) -> Result<Color, FenError> {
    match active_color {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => return Err(FenError::UnrecognizedActiveColor { color: active_color.to_string() })
    }
}

#[allow(dead_code, unused_variables)]
fn parse_castling_rights(castling_rights: &str) {
    todo!();
}

#[allow(dead_code, unused_variables)]
fn parse_enpassant_targets(enpassant_targets: &str) {
    todo!();
}

/// Parse FEN move count string, returning a `u16`.
fn parse_move_count(move_count: &str) -> Result<u16, FenError> {
    match move_count.parse() {
        Ok(val) => Ok(val),
        _ => return Err(FenError::InvalidMoveField { moves: move_count.to_string() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fen_start_pos() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let ( bitboards, active_color, halfmoves, fullmoves ) = parse_fen(fen).unwrap();
        
        assert_eq!(active_color, Color::White);
        assert_eq!(halfmoves, 0);
        assert_eq!(fullmoves, 1);
        
        assert_eq!(bitboards[Color::White as usize][Piece::Pawn as usize], 0xFF00);
        assert_eq!(bitboards[Color::White as usize][Piece::Knight as usize], 0x42);
        assert_eq!(bitboards[Color::White as usize][Piece::Bishop as usize], 0x24);
        assert_eq!(bitboards[Color::White as usize][Piece::Rook as usize], 0x81);
        assert_eq!(bitboards[Color::White as usize][Piece::Queen as usize], 0x8);
        assert_eq!(bitboards[Color::White as usize][Piece::King as usize], 0x10);

        assert_eq!(bitboards[Color::Black as usize][Piece::Pawn as usize], 0xFF000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Knight as usize], 0x4200000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Bishop as usize], 0x2400000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Rook as usize], 0x8100000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Queen as usize], 0x800000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::King as usize], 0x1000000000000000);
    }

    #[test]
    fn test_parse_fen_opening() {
        let fen = "r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq d3 0 3";
        let ( bitboards, active_color, halfmoves, fullmoves ) = parse_fen(fen).unwrap();
        
        assert_eq!(active_color, Color::Black);
        assert_eq!(halfmoves, 0);
        assert_eq!(fullmoves, 3);
        
        assert_eq!(bitboards[Color::White as usize][Piece::Pawn as usize], 0x1800e700);
        assert_eq!(bitboards[Color::White as usize][Piece::Knight as usize], 0x200002);
        assert_eq!(bitboards[Color::White as usize][Piece::Bishop as usize], 0x24);
        assert_eq!(bitboards[Color::White as usize][Piece::Rook as usize], 0x81);
        assert_eq!(bitboards[Color::White as usize][Piece::Queen as usize], 0x8);
        assert_eq!(bitboards[Color::White as usize][Piece::King as usize], 0x10);

        assert_eq!(bitboards[Color::Black as usize][Piece::Pawn as usize], 0xef001000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Knight as usize], 0x4000040000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Bishop as usize], 0x2400000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Rook as usize], 0x8100000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Queen as usize], 0x800000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::King as usize], 0x1000000000000000);
    }

    #[test]
    fn test_parse_fen_middlegame() {
        let fen = "2k2r1r/p1pq1Rpp/1pnp4/4p1p1/2N1P3/3P2P1/PPPK3P/5Q2 w - - 2 21";
        let ( bitboards, active_color, halfmoves, fullmoves ) = parse_fen(fen).unwrap();
        
        assert_eq!(active_color, Color::White);
        assert_eq!(halfmoves, 2);
        assert_eq!(fullmoves, 21);
        
        assert_eq!(bitboards[Color::White as usize][Piece::Pawn as usize], 0x10488700);
        assert_eq!(bitboards[Color::White as usize][Piece::Knight as usize], 0x4000000);
        assert_eq!(bitboards[Color::White as usize][Piece::Bishop as usize], 0);
        assert_eq!(bitboards[Color::White as usize][Piece::Rook as usize], 0x20000000000000);
        assert_eq!(bitboards[Color::White as usize][Piece::Queen as usize], 0x20);
        assert_eq!(bitboards[Color::White as usize][Piece::King as usize], 0x800);

        assert_eq!(bitboards[Color::Black as usize][Piece::Pawn as usize], 0xc50a5000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Knight as usize], 0x40000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Bishop as usize], 0);
        assert_eq!(bitboards[Color::Black as usize][Piece::Rook as usize], 0xa000000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::Queen as usize], 0x8000000000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::King as usize], 0x400000000000000);
    }

    #[test]
    fn test_parse_fen_endgame() {
        let fen = "5k2/8/6B1/3K2B1/1q6/8/3Q4/8 w - - 1 43";
        let ( bitboards, active_color, halfmoves, fullmoves ) = parse_fen(fen).unwrap();
        
        assert_eq!(active_color, Color::White);
        assert_eq!(halfmoves, 1);
        assert_eq!(fullmoves, 43);
        
        assert_eq!(bitboards[Color::White as usize][Piece::Pawn as usize], 0);
        assert_eq!(bitboards[Color::White as usize][Piece::Knight as usize], 0);
        assert_eq!(bitboards[Color::White as usize][Piece::Bishop as usize], 0x404000000000);
        assert_eq!(bitboards[Color::White as usize][Piece::Rook as usize], 0);
        assert_eq!(bitboards[Color::White as usize][Piece::Queen as usize], 0x800);
        assert_eq!(bitboards[Color::White as usize][Piece::King as usize], 0x800000000);

        assert_eq!(bitboards[Color::Black as usize][Piece::Pawn as usize], 0);
        assert_eq!(bitboards[Color::Black as usize][Piece::Knight as usize], 0);
        assert_eq!(bitboards[Color::Black as usize][Piece::Bishop as usize], 0);
        assert_eq!(bitboards[Color::Black as usize][Piece::Rook as usize], 0);
        assert_eq!(bitboards[Color::Black as usize][Piece::Queen as usize], 0x2000000);
        assert_eq!(bitboards[Color::Black as usize][Piece::King as usize], 0x2000000000000000);
    }

    #[test]
    #[should_panic(expected = "invalid number of fen fields: 1, expected 6")]
    fn test_parse_fen_invalid_fields() {
        parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
    }

    #[test]
    #[should_panic(expected = "incorrect number of ranks: 7, expected 8")]
    fn test_parse_fen_incorrect_rank_count() {
        parse_fen("rnbqkbnr/pppppppp/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "unrecognized piece char: x")]
    fn test_parse_fen_unrecognized_piece() {
        parse_fen("rnbqkbnr/pppxpppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "unrecognized square char: +, expected a valid piece char or integer in range 1..=8")]
    fn test_parse_fen_unrecognized_square() {
        parse_fen("rnbqkbnr/pppppppp/8/8/8/8/P+PPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "incorrect number of squares: 61, expected 64")]
    fn test_parse_fen_incorrect_square_count() {
        parse_fen("rnbqkbnr/pppppppp/8/32/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "unrecognized active color: Y, expected 'w' or 'b'")]
    fn test_parse_fen_unrecognized_active_color() {
        parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR Y KQkq - 0 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid move field: -1, expected unsigned 16-bit integer")]
    fn test_parse_fen_invalid_halfmove_field() {
        parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1").unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid move field: 65575, expected unsigned 16-bit integer")]
    fn test_parse_fen_invalid_fullmove_field() {
        parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 65575").unwrap();
    }

}
