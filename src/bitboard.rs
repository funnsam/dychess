use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Not};

use crate::{square::*};

/// A bitboard.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(pub u64);

impl From<Square> for Bitboard {
    /// The bitboard with a horizontal line on the given rank.
    #[inline(always)]
    fn from(value: Square) -> Self {
        Self(1 << value.to_u8())
    }
}

impl From<File> for Bitboard {
    /// The bitboard with a vertical line on the given file.
    #[inline(always)]
    fn from(value: File) -> Self {
        crate::bb_data::FILES[value as usize]
    }
}

impl From<Rank> for Bitboard {
    #[inline(always)]
    fn from(value: Rank) -> Self {
        crate::bb_data::RANKS[value as usize]
    }
}

impl Bitboard {
    /// Get the number of set bits in this bitboard.
    #[inline(always)]
    pub const fn popcnt(self) -> u32 {
        self.0.count_ones()
    }

    /// The 4 edges of the board combined.
    pub const EDGE: Self = crate::bb_data::EDGE;
}

impl Deref for Bitboard {
    type Target = u64;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Not for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}
