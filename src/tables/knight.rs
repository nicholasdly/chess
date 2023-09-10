//! Module for computing moveset lookup table, or array, for the `Knight` piece.

use std::{ fs, io };

use crate::board::{FILE_MASKS, File};

/// Lookup table for `Knight` piece moves for every square on the chessboard.
static mut KNIGHT_MOVES: [u64; 64] = [0; 64];

/// Populates moveset lookup table for the `Knight` piece.
pub fn populate_knight_moves() {
    for square in 0u8..=63 {
        unsafe {
            KNIGHT_MOVES[square as usize] = compute_knight_moves(square);
        }
    }
}

/// Writes moveset lookup table for the `Knight` piece as a constant in a specified file.
pub fn write_knight_moves(file: &mut fs::File) {
    todo!()
}

/// Given a specified square index, computes moveset bitboard for the `Knight` piece.
fn compute_knight_moves(square: u8) -> u64 {
    let bb = 1 << square;

    let s1 = (bb & !FILE_MASKS[File::A as usize]) << 15;
    let s2 = (bb & !FILE_MASKS[File::A as usize]) >> 17;
    let s3 = (bb & !(FILE_MASKS[File::A as usize] | FILE_MASKS[File::B as usize])) << 6;
    let s4 = (bb & !(FILE_MASKS[File::A as usize] | FILE_MASKS[File::B as usize])) >> 10;

    let s5 = (bb & !FILE_MASKS[File::H as usize]) << 17;
    let s6 = (bb & !FILE_MASKS[File::H as usize]) >> 15;
    let s7 = (bb & !(FILE_MASKS[File::H as usize] | FILE_MASKS[File::G as usize])) << 10;
    let s8 = (bb & !(FILE_MASKS[File::H as usize] | FILE_MASKS[File::G as usize])) >> 6;

    s1 | s2 | s3 | s4 | s5 | s6 | s7 | s8
}
