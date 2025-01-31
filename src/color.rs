use core::fmt;

use crate::square::Rank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White, Black
}

impl core::ops::Not for Color {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl Color {
    /// All of the colors with ascending indices.
    pub const ALL: [Self; 2] = [Self::White, Self::Black];

    /// Get the back rank of this side.
    #[inline(always)]
    #[must_use]
    pub const fn back_rank(self) -> Rank {
        match self {
            Self::White => Rank::_1,
            Self::Black => Rank::_8,
        }
    }

    /// Map `self` to either `'w'` or `'b'`.
    #[inline(always)]
    #[must_use]
    pub const fn to_char(self) -> char {
        match self {
            Self::White => 'w',
            Self::Black => 'b',
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
