mod fen;

/// Array of bitmasks for each rank of the chessboard.
pub const RANK_MASKS: [u64; 8] = [
    0x00000000000000FF,
    0x000000000000FF00,
    0x0000000000FF0000,
    0x00000000FF000000,
    0x000000FF00000000,
    0x0000FF0000000000,
    0x00FF000000000000,
    0xFF00000000000000
];

/// Array of bitmasks for each file of the chessboard.
pub const FILE_MASKS: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080
];

/// Represents a rank of the chessboard.
pub enum Rank { One, Two, Three, Four, Five, Six, Seven, Eight }

/// Represents a file of the chessboard.
pub enum File { A, B, C, D, E, F, G, H }

/// Represents a color of a chess piece.
#[derive(Debug, PartialEq)]
pub enum Color { White, Black }

/// Represents a type of chess piece.
pub enum Piece { Pawn, Knight, Bishop, Rook, Queen, King }

/// Represents a player's castling rights.
#[derive(Debug, PartialEq)]
pub enum CastleRights { None, Kingside, Queenside, Both }

/// Represents a chessboard.
#[allow(dead_code)]
pub struct Board {
    bitboards: [[u64; 6]; 2],
    active_color: Color,
    castling_rights: [CastleRights; 2],
    halfmoves: u16,
    fullmoves: u16
}

impl Board {

    /// Creates a new chessboard initialized with pieces in their starting positions.
    pub fn start_pos() -> Board {
        Board::from_fen(fen::FEN_START)
    }

    /// Creates a new chessboard initialized from a FEN string.
    pub fn from_fen(fen: &str) -> Board {
        let (
            bitboards,
            active_color,
            castling_rights,
            halfmoves,
            fullmoves
        ) = fen::parse_fen(fen).unwrap();

        Board { bitboards, active_color, castling_rights, halfmoves, fullmoves }
    }

    pub fn apply_move(&self) {
        todo!();
    }

}
