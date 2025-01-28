use crate::{magic, prelude::*, rays};

/// Get the rays of a rook on the specified square.
pub fn rays(square: Square) -> Bitboard {
    rays::ROOK[square.to_usize()]
}

/// Get the possible moves of a rook given a list of pieces on the board, assuming they're enemy
/// pieces.
pub fn moves(square: Square, blockers: Bitboard) -> Bitboard {
    let (magic, table) = magic::ROOK[square.to_usize()];
    table[((blockers & magic.mask).wrapping_mul(magic.mul) >> (64 - magic.bits)) as usize]
}
