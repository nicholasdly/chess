//! Module for computing moveset lookup tables, or arrays, for the `Pawn` piece.

use std::{ fs, io };

use crate::board::{FILE_MASKS, RANK_MASKS, File, Rank, Color};

/// Lookup table for `Pawn` piece quiet moves for every square on the chessboard.
static mut PAWN_QUIET_MOVES: [[u64; 64]; 2] = [[0; 64]; 2];

/// Lookup table for `Pawn` piece attacking moves for every square on the chessboard.
static mut PAWN_ATTACKING_MOVES: [[u64; 64]; 2] = [[0; 64]; 2];

/// Populates moveset lookup tables for the `Pawn` piece.
pub fn populate_pawn_moves() {
    for square in 0u8..=63 {
        unsafe {
            PAWN_QUIET_MOVES[Color::White as usize][square as usize] = compute_pawn_quiet_moves(square, Color::White);
            PAWN_QUIET_MOVES[Color::Black as usize][square as usize] = compute_pawn_quiet_moves(square, Color::Black);
            PAWN_ATTACKING_MOVES[Color::White as usize][square as usize] = compute_pawn_attacking_moves(square, Color::White);
            PAWN_ATTACKING_MOVES[Color::Black as usize][square as usize] = compute_pawn_attacking_moves(square, Color::Black);
        }
    }
}

/// Writes moveset lookup tables for the `Pawn` piece as constants in a specified file.
pub fn write_pawn_moves(file: &mut fs::File) {
    todo!()
}

/// Given a specified square index and piece color, computes quiet moveset bitboard for the `Pawn`
/// piece.
fn compute_pawn_quiet_moves(square: u8, color: Color) -> u64 {
    let bb = 1 << square;
    
    let s1;
    let s2;

    if color == Color::White {
        s1 = bb << 8;
        s2 = (bb & RANK_MASKS[Rank::Two as usize]) << 16;
    } else {
        s1 = bb >> 8;
        s2 = (bb & RANK_MASKS[Rank::Seven as usize]) >> 16;
    }

    s1 | s2
}

/// Given a specified square index and piece color, computes attacking moveset bitboard for the
/// `Pawn` piece.
fn compute_pawn_attacking_moves(square: u8, color: Color) -> u64 {
    let bb = 1 << square;

    let s1;
    let s2;

    if color == Color::White {
        s1 = (bb & !FILE_MASKS[File::A as usize]) << 7;
        s2 = (bb & !FILE_MASKS[File::H as usize]) << 9;
    } else {
        s1 = (bb & !FILE_MASKS[File::A as usize]) >> 9;
        s2 = (bb & !FILE_MASKS[File::H as usize]) >> 7;
    }

    s1 | s2
}
