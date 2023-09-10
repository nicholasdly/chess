//! Module for determining the least significant 1 bit (LS1B) or the most significant 1 bit (MS1B)
//! in an integer such as bitboards, otherwise known bitscan.
//! 
//! For more information, see the [Chess Programming Wiki](https://www.chessprogramming.org/BitScan).

use std::fmt;

/// Represents possible Errors encountered while executing bitscan.
pub enum BitscanError {
    InvalidInteger { integer: u64 }
}

impl fmt::Debug for BitscanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitscanError::InvalidInteger { integer } => {
                writeln!(f, "invalid value: {integer}, expected nonzero integer")
            }
        }
    }
}

/// Computes the index of the least significant 1 bit (LS1B).
pub fn bitscan_forward(i: u64) -> Result<u8, BitscanError> {
    match i != 0 {
        true => Ok(i.trailing_zeros() as u8),
        false => Err(BitscanError::InvalidInteger { integer: i })
    }
}

/// Computes the index of the most significant 1 bit (MS1B).
pub fn bitscan_reverse(i: u64) -> Result<u8, BitscanError> {
    match i != 0 {
        true => Ok(63 - i.leading_zeros() as u8),
        false => Err(BitscanError::InvalidInteger { integer: i })
    }
}
