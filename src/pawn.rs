use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/pawn.rs"));

/// Get the possible advancing moves of a pawn.
pub fn advances(color: Color, square: Square) -> Bitboard {
    ADVANCES[color as usize][square.to_usize()]
}

/// Get the possible capturing moves of a pawn.
pub fn captures(color: Color, square: Square) -> Bitboard {
    CAPTURES[color as usize][square.to_usize()]
}
