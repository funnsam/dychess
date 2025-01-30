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

    #[inline(always)]
    pub fn back_rank(self) -> Rank {
        match self {
            Color::White => Rank::_1,
            Color::Black => Rank::_8,
        }
    }
}
