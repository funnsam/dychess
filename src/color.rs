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

    /// Convert an index to a color.
    ///
    /// # Safety
    /// The index must be valid.
    #[inline(always)]
    #[must_use]
    pub unsafe fn from_index_unchecked(idx: u8) -> Self {
        // SAFETY: up to caller
        unsafe {
            core::mem::transmute::<u8, Self>(idx)
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl TryFrom<u8> for Color {
    type Error = ();

    #[inline(always)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // SAFETY: checked before
        (value <= Self::Black as _).then(|| unsafe {
            Self::from_index_unchecked(value)
        }).ok_or(())
    }
}

impl From<bool> for Color {
    /// Convert a boolean to a color. `false` is white and `true` is black.
    ///
    /// ```
    /// # use dychess::prelude::*;
    /// #
    /// assert_eq!(Color::from_bool(false), Color::White);
    /// assert_eq!(Color::from_bool(true), Color::Black);
    /// ```
    #[inline(always)]
    fn from(value: bool) -> Self {
        // SAFETY: `bool`s are only 0..=1 like colors
        unsafe {
            core::mem::transmute(value)
        }
    }
}
