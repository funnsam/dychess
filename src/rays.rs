use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/rays.rs"));

/// Get the rays of a bishop on the specified square.
pub fn bishop_rays(square: Square) -> Bitboard { BISHOP_RAYS[square.to_usize()] }

/// Get the rays of a rook on the specified square.
pub fn rook_rays(square: Square) -> Bitboard { ROOK_RAYS[square.to_usize()] }

/// Get the rays of a queen on the specified square. This is the same as the bitboard of bishop
/// and rook ORed together.
pub fn queen_rays(square: Square) -> Bitboard { bishop_rays(square) | rook_rays(square) }
