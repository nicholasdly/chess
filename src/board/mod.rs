pub mod bitboards;
pub mod fen;

/// Represents a color of a chess piece.
#[derive(Debug, PartialEq)]
pub enum Color { White, Black }

/// Represents a type of chess piece.
pub enum Piece { Pawn, Knight, Bishop, Rook, Queen, King }

/// Represents a chessboard.
#[allow(dead_code)]
pub struct Board {
    bitboards: bitboards::Bitboards,
    active_color: Color,
    halfmoves: u16,
    fullmoves: u16
}

impl Board {

    /// Creates a new chessboard initialized with pieces in their starting positions.
    pub fn start_pos() -> Board {
        Board::from_fen(fen::START_POSITION)
    }

    /// Creates a new chessboard initialized from a FEN string.
    pub fn from_fen(fen: &str) -> Board {
        let (
            bitboards,
            active_color,
            halfmoves,
            fullmoves
        ) = fen::parse_fen(fen).unwrap();

        Board { bitboards, active_color, halfmoves, fullmoves }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_start_pos() {
        let board = Board::start_pos();

        assert_eq!(board.active_color, Color::White);
        assert_eq!(board.halfmoves, 0);
        assert_eq!(board.fullmoves, 1);

        assert_eq!(board.bitboards[Color::White as usize][Piece::Pawn as usize], 0xFF00);
        assert_eq!(board.bitboards[Color::White as usize][Piece::Knight as usize], 0x42);
        assert_eq!(board.bitboards[Color::White as usize][Piece::Bishop as usize], 0x24);
        assert_eq!(board.bitboards[Color::White as usize][Piece::Rook as usize], 0x81);
        assert_eq!(board.bitboards[Color::White as usize][Piece::Queen as usize], 0x8);
        assert_eq!(board.bitboards[Color::White as usize][Piece::King as usize], 0x10);

        assert_eq!(board.bitboards[Color::Black as usize][Piece::Pawn as usize], 0xFF000000000000);
        assert_eq!(board.bitboards[Color::Black as usize][Piece::Knight as usize], 0x4200000000000000);
        assert_eq!(board.bitboards[Color::Black as usize][Piece::Bishop as usize], 0x2400000000000000);
        assert_eq!(board.bitboards[Color::Black as usize][Piece::Rook as usize], 0x8100000000000000);
        assert_eq!(board.bitboards[Color::Black as usize][Piece::Queen as usize], 0x800000000000000);
        assert_eq!(board.bitboards[Color::Black as usize][Piece::King as usize], 0x1000000000000000);
    }

}
