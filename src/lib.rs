#![no_std]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
)]
#![allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::inline_always,
    clippy::unreadable_literal,
)]

pub mod bitboard;
pub mod board;
pub mod castle_rights;
pub mod chess_move;
pub mod color;
pub mod piece;
pub mod square;

pub mod pawn;
pub mod knight;
pub mod bishop;
pub mod rook;
pub mod queen;
pub mod king;

pub mod prelude {
    pub use crate::bitboard::*;
    pub use crate::board::*;
    pub use crate::castle_rights::*;
    pub use crate::chess_move::*;
    pub use crate::color::*;
    pub use crate::piece::*;
    pub use crate::square::*;
    pub use crate::{pawn, knight, bishop, rook, queen, king};
}

pub(crate) mod bb_data {
    use crate::bitboard::Bitboard;

    include!(concat!(env!("OUT_DIR"), "/bitboard.rs"));
}

pub(crate) mod rays {
    use crate::bitboard::Bitboard;

    include!(concat!(env!("OUT_DIR"), "/rays.rs"));
}

pub(crate) mod magic {
    use crate::prelude::*;

    include!(concat!(env!("OUT_DIR"), "/magic.rs"));

    #[derive(Debug, Clone, Copy)]
    pub struct Magic {
        pub mask: Bitboard,
        pub mul: u64,
        pub bits: u8,
    }
}
