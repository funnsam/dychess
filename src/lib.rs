#![doc = include_str!("../README.md")]

pub mod bitboard;
pub mod board;
pub mod castle_rights;
pub mod chess_move;
pub mod color;
pub mod king;
pub mod knight;
pub mod pawn;
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
