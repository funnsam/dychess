use crate::prelude::*;

/// Get the rays of a queen on the specified square.
pub fn rays(square: Square) -> Bitboard {
    bishop::rays(square) | rook::rays(square)
}

/// Get the possible moves of a queen given a list of pieces on the board, assuming they're enemy
/// pieces.
pub fn moves(square: Square, blockers: Bitboard) -> Bitboard {
    bishop::moves(square, blockers) | rook::moves(square, blockers)
}
