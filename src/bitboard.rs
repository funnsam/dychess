use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref};

/// A bitboard.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// Get the number of set bits in this bitboard.
    #[inline(always)]
    pub fn popcnt(self) -> u32 {
        self.0.count_ones()
    }
}

impl Deref for Bitboard {
    type Target = u64;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
        self.0 ^= rhs.0
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
