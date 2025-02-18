use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Not};

use crate::square::{File, Rank, Square};

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
        Self(0x0101010101010101_u64 << value as u8)
    }
}

impl From<Rank> for Bitboard {
    /// The bitboard with a horizontal line on the given rank.
    #[inline(always)]
    fn from(value: Rank) -> Self {
        Self(0xff << (value as u8 * 8))
    }
}

impl TryFrom<Bitboard> for Square {
    type Error = u32;

    #[inline(always)]
    fn try_from(bb: Bitboard) -> Result<Self, Self::Error> {
        let popcnt = bb.popcnt();
        if popcnt != 1 { return Err(popcnt) };

        Ok(bb.first_square().unwrap())
    }
}

impl Bitboard {
    /// Get if this bitboard is empty.
    #[inline(always)]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Get the first square in this bitboard, or [None] if it is empty.
    #[inline(always)]
    #[must_use]
    pub const fn first_square(self) -> Option<Square> {
        if self.is_empty() { return None };

        Some(Square::from_index(self.0.trailing_zeros() as _))
    }

    /// Get the last square in this bitboard, or [None] if it is empty.
    #[inline(always)]
    #[must_use]
    pub const fn last_square(self) -> Option<Square> {
        if self.is_empty() { return None };

        Some(Square::from_index(63 - self.0.leading_zeros() as u8))
    }

    /// Get the number of set bits in this bitboard.
    #[inline(always)]
    #[must_use]
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
        self.0 &= rhs.0;
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
        self.0 ^= rhs.0;
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct BitboardIter(Bitboard);

impl BitboardIter {
    /// Get whether the iterator is empty or not. This is a faster way to do `self.len() == 0`.
    #[inline(always)]
    #[must_use]
    pub const fn had_emptied(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the squares that aren't iterated through yet.
    #[inline(always)]
    #[must_use]
    pub const fn remainder(&self) -> Bitboard {
        self.0
    }
}

impl Iterator for BitboardIter {
    type Item = Square;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        (!self.had_emptied()).then(|| {
            let tz = self.0.0.trailing_zeros();
            self.0.0 &= self.0.0 - 1;

            unsafe { Square::from_index_unchecked(tz as u8) }
        })
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let popcnt = self.0.popcnt() as usize;
        (popcnt, Some(popcnt))
    }
}

impl core::iter::FusedIterator for BitboardIter {}
impl ExactSizeIterator for BitboardIter {}
