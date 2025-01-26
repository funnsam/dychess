use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/pawn_moves.rs"));

pub fn pawn_advances(color: Color, square: Square) -> Bitboard {
    PAWN_ADVANCES[color as usize][square.to_usize()]
}

pub fn pawn_captures(color: Color, square: Square) -> Bitboard {
    PAWN_CAPTURES[color as usize][square.to_usize()]
}
