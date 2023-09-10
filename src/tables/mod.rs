//! Module for initializing lookup tables. Used in build script to pre-compute all lookup tables
//! in compile time for use by the engine in run time.

mod bitscan;

pub mod sliding;
pub mod pawn;
pub mod knight;
pub mod king;
