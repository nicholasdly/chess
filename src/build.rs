//! This file is a build script, and is compiled and executed just before the package is built.
//! 
//! The purpose of this file is to pre-compute moveset lookup tables for every chess piece in, and
//! store the data as constants in `move_tables.rs`, so that it can be easily referenced during
//! move generation.

use crate::tables::king::*;
use crate::tables::knight::*;
use crate::tables::pawn::*;
use crate::tables::sliding::*;

use std::env;
use std::fs::File;
use std::path::Path;

#[allow(dead_code)]
fn main() {
    populate_king_moves();
    populate_knight_moves();
    populate_pawn_moves();
    populate_first_rank_moves();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("move_tables.rs");
    let mut f = File::create(&dest_path).unwrap();

    write_king_moves(&mut f);
    write_knight_moves(&mut f);
    write_pawn_moves(&mut f);
    write_first_rank_moves(&mut f);
}
