/// A square on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    /// Make a new square from a file and rank.
    pub const fn new(file: File, rank: Rank) -> Self {
        Self(((rank as u8) << 3) | (file as u8))
    }

    /// Make a new square from an index.
    ///
    /// # Panics
    /// It will panic if `idx > 63`.
    pub const fn from_index(idx: u8) -> Self {
        assert!(idx < 64);
        Self(idx)
    }

    /// Converts this square to an `u8`.
    pub const fn to_u8(self) -> u8 { self.0 as _ }

    /// Converts this square to an `usize`.
    pub const fn to_usize(self) -> usize { self.0 as _ }
}

/// A file (or column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A, B, C, D, E, F, G, H
}

/// A rank (or row).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    _1, _2, _3, _4, _5, _6, _7, _8
}
