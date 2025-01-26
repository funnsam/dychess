#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White, Black
}

impl core::ops::Not for Color {
    type Output = Self;

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
}
