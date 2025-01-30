#![doc = include_str!("../README.md")]

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
    use crate::bitboard::*;

    include!(concat!(env!("OUT_DIR"), "/bitboard.rs"));
}

pub(crate) mod rays {
    use crate::bitboard::*;

    include!(concat!(env!("OUT_DIR"), "/rays.rs"));
}

pub(crate) mod magic {
    use crate::prelude::*;

    include!(concat!(env!("OUT_DIR"), "/magic.rs"));

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct Magic {
        pub mask: Bitboard,
        pub mul: u64,
        pub bits: u8,
    }
}
