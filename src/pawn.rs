use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/pawn.rs"));

/// Get the possible advancing moves of a pawn.
#[inline(always)]
pub fn advances(color: Color, square: Square, blockers: Bitboard) -> Bitboard {
    // SAFETY: `square` < 64
    let adv = unsafe { *ADVANCES[color as usize].get_unchecked(square.to_usize()) };
    (if (adv & blockers).is_empty() {
        adv
    } else {
        adv & !Bitboard::from(Rank::_4) & !Bitboard::from(Rank::_5)
    }) & !blockers
}

/// Get the possible capturing moves of a pawn.
#[inline(always)]
pub fn captures(color: Color, square: Square) -> Bitboard {
    // SAFETY: `square` < 64
    unsafe { *CAPTURES[color as usize].get_unchecked(square.to_usize()) }
}

#[inline(always)]
pub fn double_pushes(color: Color) -> Bitboard {
    DOUBLE_PUSHES[color as usize]
}

#[inline(always)]
pub fn ep_targets(color: Color) -> Bitboard {
    match color {
        Color::White => Rank::_6.into(),
        Color::Black => Rank::_3.into(),
    }
}
