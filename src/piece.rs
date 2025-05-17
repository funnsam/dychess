use core::fmt;

use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl Piece {
    /// All of the pieces with ascending indices.
    pub const ALL: [Self; 6] = [
        Self::Pawn, Self::Knight, Self::Bishop, Self::Rook, Self::Queen, Self::King
    ];

    /// All of the pieces that a pawn can promote to with ascending indices.
    pub const PROMOTE_TO: [Self; 4] = [
        Self::Knight, Self::Bishop, Self::Rook, Self::Queen
    ];

    /// Convert a piece to a unique uppercase character. This is the same as the ones used in FENs.
    #[inline(always)]
    #[must_use]
    pub const fn to_uppercase_char(self) -> char {
        self.to_lowercase_char().to_ascii_uppercase()
    }

    /// Convert a piece to a unique lowercase character. This is the same as the ones used in FENs.
    #[inline(always)]
    #[must_use]
    pub const fn to_lowercase_char(self) -> char {
        match self {
            Self::Pawn => 'p',
            Self::Knight => 'n',
            Self::Bishop => 'b',
            Self::Rook => 'r',
            Self::Queen => 'q',
            Self::King => 'k',
        }
    }

    /// Convert a piece to a unique character. This is the same as the ones used in FENs.
    #[inline(always)]
    #[must_use]
    pub const fn to_char(self, color: Color) -> char {
        match color {
            Color::White => self.to_uppercase_char(),
            Color::Black => self.to_lowercase_char(),
        }
    }

    /// Convert an index to a piece.
    ///
    /// # Safety
    /// The index must be valid.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_index_unchecked(idx: u8) -> Self {
        // SAFETY: up to caller
        unsafe {
            core::mem::transmute::<u8, Self>(idx)
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_lowercase_char())
    }
}

impl TryFrom<u8> for Piece {
    type Error = ();

    #[inline(always)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // SAFETY: checked before
        (value <= Self::King as _).then(|| unsafe {
            Self::from_index_unchecked(value)
        }).ok_or(())
    }
}
