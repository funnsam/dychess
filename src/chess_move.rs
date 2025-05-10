use core::{fmt, num::NonZeroU16};

use crate::{piece::Piece, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move(NonZeroU16);

impl Move {
    /// Create a new move.
    ///
    /// # Promotion
    /// The promotion will be removed if it's a pawn due to internal representation. Note that this
    /// might change to other behavior on future versions.
    ///
    /// ```
    /// # use dychess::prelude::*;
    /// #
    /// let mov = Move::new(Square::E2, Square::E4, Some(Piece::Pawn));
    /// assert_eq!(mov.promotion(), None);
    /// ```
    ///
    /// # Panics
    /// Panics if `from` and `to` are the `A1` square, and promotion is `None` or
    /// `Some(Piece::Pawn)`. This is due to this type uses [`NonZeroU16`] internally, so that
    /// `Option<Move>` is efficient.
    ///
    /// ```should_panic
    /// # use dychess::prelude::*;
    /// #
    /// // This will not work
    /// Move::new(Square::A1, Square::A1, Some(Piece::Pawn));
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        let promotion = if let Some(p) = promotion { p } else { Piece::Pawn };

        if matches!((from, to, promotion), (Square::A1, Square::A1, Piece::Pawn)) {
            panic!("null-valued move passed into `Move::new`");
        }

        unsafe {
            Self::encode(from, to, promotion)
        }
    }

    /// Create a new move, without checking for null-valued moves.
    ///
    /// # Safety
    /// See [`Self::new`] Panics section.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn new_unchecked(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        let promotion = if let Some(p) = promotion { p } else { Piece::Pawn };

        unsafe {
            Self::encode(from, to, promotion)
        }
    }

    /// Construct a new move directly from a [`NonZeroU16`].
    ///
    /// # Layout
    /// - Bits `12..=15` are the piece to promote to (0 means `None`).
    /// - Bits `6..=11` are the destination square.
    /// - Bits `0..=5` are the source square.
    ///
    /// # Safety
    /// The promotion field must be in the range of `0..6`.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_value(val: NonZeroU16) -> Self {
        Self(val)
    }

    #[inline(always)]
    #[must_use]
    const unsafe fn encode(from: Square, to: Square, promotion: Piece) -> Self {
        let val = ((promotion as u16) << 12)
            | ((to.to_u8() as u16) << 6)
            | (from.to_u8() as u16);

        unsafe {
            Self(NonZeroU16::new_unchecked(val))
        }
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
    pub const fn promotion(&self) -> Option<Piece> {
        let idx = self.0.get() >> 12;

        if idx != 0 {
            // SAFETY: valid `Move`s has valid promotion field
            Some(unsafe { core::mem::transmute(idx as u8) })
        } else {
            None
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.from(), self.to(), match self.promotion() {
            None | Some(Piece::Pawn) => "",
            Some(Piece::Knight) => "n",
            Some(Piece::Bishop) => "b",
            Some(Piece::Rook) => "r",
            Some(Piece::Queen) => "q",
            Some(Piece::King) => "k",
        })
    }
}

impl From<Move> for NonZeroU16 {
    fn from(value: Move) -> Self {
        value.0
    }
}

impl From<Move> for u16 {
    fn from(value: Move) -> Self {
        value.0.get()
    }
}
