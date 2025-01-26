//! # Dychess — (hopefully) fast chess library for chess engines
//! This crate was created to potentially make my chess engine Dysprosium faster by using a much
//! better chess library that is optimized for better chess engine performance instead of the
//! generic `chess` crate, and opening up the possibility of playing Fischer Random Chess as well.

pub mod castle_rights;
pub mod chess_move;
pub mod color;
pub mod bitboard;
pub mod board;
pub mod pawn_moves;
pub mod piece;
pub mod rays;
pub mod square;

pub mod prelude {
    pub use crate::castle_rights::*;
    pub use crate::chess_move::*;
    pub use crate::color::*;
    pub use crate::bitboard::*;
    pub use crate::board::*;
    pub use crate::piece::*;
    pub use crate::square::*;
}

mod bb_data {
    use crate::bitboard::*;

    include!(concat!(env!("OUT_DIR"), "/bitboard.rs"));
}
