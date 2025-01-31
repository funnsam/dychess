use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/pawn.rs"));

/// Get the possible advancing moves of a pawn.
#[inline(always)]
#[must_use]
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
#[must_use]
pub fn captures(color: Color, square: Square) -> Bitboard {
    // SAFETY: `square` < 64
    unsafe { *CAPTURES[color as usize].get_unchecked(square.to_usize()) }
}

#[inline(always)]
#[must_use]
pub fn double_pushes(color: Color) -> Bitboard {
    match color {
        Color::White => Bitboard::from(Rank::_2) | Bitboard::from(Rank::_4),
        Color::Black => Bitboard::from(Rank::_7) | Bitboard::from(Rank::_5),
    }
}

#[inline(always)]
#[must_use]
pub fn double_push_to(color: Color) -> Bitboard {
    match color {
        Color::White => Bitboard::from(Rank::_4),
        Color::Black => Bitboard::from(Rank::_5),
    }
}

#[inline(always)]
#[must_use]
pub const fn ep_target_rank(color: Color) -> Rank {
    match color {
        Color::White => Rank::_6,
        Color::Black => Rank::_3,
    }
}

#[inline(always)]
#[must_use]
pub fn ep_targets(color: Color) -> Bitboard {
    ep_target_rank(color).into()
}
