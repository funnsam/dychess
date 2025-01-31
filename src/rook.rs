use crate::{magic, prelude::*, rays};

/// Get the rays of a rook on the specified square.
#[inline(always)]
#[must_use]
pub fn rays(square: Square) -> Bitboard {
    rays::ROOK[square.to_usize()]
}

/// Get the possible moves of a rook given a list of pieces on the board, assuming they're enemy
/// pieces.
#[inline(always)]
#[must_use]
pub fn moves(square: Square, blockers: Bitboard) -> Bitboard {
    magic::rook_moves(square, blockers)
}
