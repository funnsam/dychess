use core::{fmt, num::NonZeroU16};

use crate::{piece::Piece, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move(NonZeroU16);

impl Move {
    /// Create a new move.
    ///
    /// ```
    /// use dychess::prelude::*;
    ///
    /// let mov = Move::new(Square::E2, Square::E4, Some(Piece::Pawn));
    /// assert_eq!(mov.promotion(), None);
    /// ```
    ///
    /// # Promotion
    /// The promotion will be removed if it's a pawn due to internal representation. Note that this
    /// might change to other behavior on future versions.
    ///
    /// # Panics
    /// Panics if `from` and `to` are the `A1` square, and promotion is `None` or
    /// `Some(Piece::Pawn)`. This is due to this type uses [`NonZeroU16`] internally, so that
    /// `Option<Move>` is efficient.
    ///
    /// ```should_panic
    /// # use dychess::prelude::*;
    /// #
    /// Move::new(Square::A1, Square::A1, Some(Piece::Pawn));
    /// ```
    ///
    /// ```
    /// # use dychess::prelude::*;
    /// #
    /// assert_eq!(size_of::<Option<Move>>(), size_of::<Move>());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        let promotion = if let Some(p) = promotion { p } else { Piece::Pawn };

        let val = ((promotion as u16) << 12)
            | ((to.to_u8() as u16) << 6)
            | (from.to_u8() as u16);
        Self(NonZeroU16::new(val).expect("null-valued move passed into `Move::new`"))
    }

    #[inline(always)]
    #[must_use]
    pub const fn from(&self) -> Square {
        Square::from_index(self.0.get() as u8 & 63)
    }

    #[inline(always)]
    #[must_use]
    pub const fn to(&self) -> Square {
        Square::from_index((self.0.get() >> 6) as u8 & 63)
    }

    #[inline(always)]
    #[must_use]
    pub fn promotion(&self) -> Option<Piece> {
        let promotion = unsafe { *Piece::ALL.get_unchecked(self.0.get() as usize >> 12) };

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
