//! Module for computing moveset lookup table, or array, for sliding pieces on the first rank.

use std::{ fs, io };
use super::bitscan::{ bitscan_forward, bitscan_reverse };

/// Lookup table for sliding piece moves for every square and occupancy on the first rank.
/// 
/// Although possible to store sliding piece moves for every square and occupancy on *every* file,
/// rank, and diagonal, a significant amount of memory is saved by the fact we can represent any
/// file, rank, and diagonal on the first rank via bit-twiddling, or more specifically flips and
/// rotation.
static mut FIRST_RANK_MOVES: [[u8; 256]; 8] = [[0; 256]; 8];

/// Populates moveset lookup table for sliding pieces on the first rank.
pub fn populate_first_rank_moves() {
    for square in 0u8..8 {
        for occupancy in 0u8..=255 {
            unsafe {
                FIRST_RANK_MOVES[square as usize][occupancy as usize] = compute_first_rank_moves(square, occupancy);
            }
        }
    }
}

/// Writes moveset lookup table for sliding pieces on the first rank as a constant in a specified
/// file.
pub fn write_first_rank_moves(file: &mut fs::File) {
    todo!()
}

/// Given a specified square index, computes moveset bitboard for a sliding piece on the first rank
/// of specified occupancy.
fn compute_first_rank_moves(square: u8, occupancy: u8) -> u8 {    
    let left_ray = |x: u8| -> u8 { x - 1 };
    let right_ray = |x: u8| -> u8 { !x & !(x - 1) };

    let mut left_attacks = left_ray(1 << square);
    let left_blockers = left_attacks & occupancy;
    if left_blockers > 0 {
        let leftmost = 1 << bitscan_reverse(left_blockers as u64).unwrap();
        left_attacks ^= left_ray(leftmost);
    }

    let mut right_attacks = right_ray(1 << square);
    let right_blockers = right_attacks & occupancy;
    if right_blockers > 0 {
        let rightmost = 1 << bitscan_forward(right_blockers as u64).unwrap();
        right_attacks ^= right_ray(rightmost);
    }

    left_attacks ^ right_attacks
}
