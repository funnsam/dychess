use crate::{magic, prelude::*, rays};

/// Get the rays of a bishop on the specified square.
#[inline(always)]
pub fn rays(square: Square) -> Bitboard {
    // SAFETY: `square` < 64
    unsafe { *rays::BISHOP.get_unchecked(square.to_usize()) }
}

/// Get the possible moves of a bishop given a list of pieces on the board, assuming they're enemy
/// pieces.
#[inline(always)]
pub fn moves(square: Square, blockers: Bitboard) -> Bitboard {
    magic::bishop_moves(square, blockers)
}
