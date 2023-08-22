
/// Represents a color of a chess piece.
pub enum Color {
    White,
    Black
}

/// Represents a type of chess piece.
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

/// Represents a chessboard.
pub struct Board {
    bitboards: [[u64; 6]; 2]
}

impl Board {

    /// Creates a new chessboard initialized with pieces in their starting positions.
    pub fn start_pos() -> Board {
        Board {
            bitboards: [[
                0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000
            ],[
                0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
                0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
                0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
            ]]
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_start_pos() {
        let board = Board::start_pos();

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
        assert_eq!(board.bitboards[Color::Black as usize][Piece::Queen as usize], 0x0800000000000000);
        assert_eq!(board.bitboards[Color::Black as usize][Piece::King as usize], 0x1000000000000000);
    }

}
