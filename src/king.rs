use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/king.rs"));

/// Get the possible moves of a king on a certain square.
pub fn moves(square: Square) -> Bitboard {
    MOVES[square.to_usize()]
}

/// Get the squares required not to be under attacked when castling.
pub fn castle_path(color: Color, king_file: File, rook_file: File) -> Bitboard {
    CASTLE_PATH[color as usize][king_file as usize][(king_file > rook_file) as usize]
}

/// Get the squares required to be empty when castling.
pub fn castle_clearance(color: Color, king_file: File, rook_file: File) -> Bitboard {
    CASTLE_CLEARANCE[color as usize][king_file as usize][(king_file > rook_file) as usize]
}
