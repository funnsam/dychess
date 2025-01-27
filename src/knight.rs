use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/knight.rs"));

/// Get the possible moves of a knight on a certain square.
pub fn moves(square: Square) -> Bitboard {
    MOVES[square.to_usize()]
}
