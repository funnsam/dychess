use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/pawn.rs"));

/// Get the possible advancing moves of a pawn.
pub fn advances(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
    let adv = ADVANCES[color as usize][square.to_usize()];
    (if (adv & blockers).is_empty() {
        adv
    } else {
        adv & !Bitboard::from(Rank::_4) & !Bitboard::from(Rank::_5)
    }) & !blockers
}

/// Get the possible capturing moves of a pawn.
pub fn captures(color: Color, square: Square) -> Bitboard {
    CAPTURES[color as usize][square.to_usize()]
}

pub fn double_pushes(color: Color) -> Bitboard {
    DOUBLE_PUSHES[color as usize]
}
