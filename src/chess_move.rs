use core::fmt;

use crate::{piece::Piece, square::Square};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move(u16);

impl Move {
    /// Create a new move.
    ///
    /// # Promotion
    /// The promotion will be removed if it's a pawn due to internal representation. Note that this
    /// might change to other behavior on future versions.
    ///
    /// ```
    /// use dychess::prelude::*;
    ///
    /// let mov = Move::new(Square::A1, Square::A1, Some(Piece::Pawn));
    /// assert_eq!(mov.promotion(), None);
    /// ```
    #[inline(always)]
    pub const fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        if let Some(promotion) = promotion {
            Self(((promotion as u16) << 12) | ((to.to_u8() as u16) << 6) | (from.to_u8() as u16))
        } else {
            Self(((to.to_u8() as u16) << 6) | (from.to_u8() as u16))
        }
    }

    #[inline(always)]
    pub const fn from(&self) -> Square { Square::from_index(self.0 as u8 & 63) }

    #[inline(always)]
    pub const fn to(&self) -> Square { Square::from_index((self.0 >> 6) as u8 & 63) }

    #[inline(always)]
    pub fn promotion(&self) -> Option<Piece> {
        let promotion = unsafe { *Piece::ALL.get_unchecked(self.0 as usize >> 12) };

        (promotion != Piece::Pawn).then_some(promotion)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.from(), self.to(), match self.promotion() {
            Some(Piece::Pawn) => "p",
            Some(Piece::Knight) => "n",
            Some(Piece::Bishop) => "b",
            Some(Piece::Rook) => "r",
            Some(Piece::Queen) => "q",
            Some(Piece::King) => "k",
            None => "",
        })
    }
}
