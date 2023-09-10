//! Module for computing moveset lookup table, or array, for the `King` piece.

use std::{ fs, io };

use crate::board::{FILE_MASKS, File};

/// Lookup table for `King` piece moves for every square on the chessboard.
static mut KING_MOVES: [u64; 64] = [0; 64];

/// Populates moveset lookup table for the `King` piece.
pub fn populate_king_moves() {
    for square in 0u8..=63 {
        unsafe {
            KING_MOVES[square as usize] = compute_king_moves(square);
        }
    }
}

/// Writes moveset lookup table for the `King` piece as a constant in a specified file.
pub fn write_king_moves(file: &mut fs::File) {
    todo!()
}

/// Given a specified square index, computes moveset bitboard for the `King` piece.
fn compute_king_moves(square: u8) -> u64 {
    let bb = 1 << square;

    let n = bb << 8;
    let s = bb >> 8;

    let nw = (bb & FILE_MASKS[File::A as usize]) << 7;
    let w = (bb & FILE_MASKS[File::A as usize]) >> 1;
    let sw = (bb & FILE_MASKS[File::A as usize]) >> 9;

    let ne = (bb & FILE_MASKS[File::H as usize]) << 9;
    let e = (bb & FILE_MASKS[File::H as usize]) << 1;
    let se = (bb & FILE_MASKS[File::H as usize]) >> 7;

    n | s | nw | w | sw | ne | e | se
}
