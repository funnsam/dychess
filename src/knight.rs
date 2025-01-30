use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/knight.rs"));

/// Get the possible moves of a knight on a certain square.
#[inline(always)]
pub fn moves(square: Square) -> Bitboard {
    // SAFETY: `square` < 64
    unsafe { *MOVES.get_unchecked(square.to_usize()) }
}
