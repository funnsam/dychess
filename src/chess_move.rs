use crate::{piece::Piece, square::Square};

pub struct Move {
    from: Square,
    to: Square,
    /// A pawn if none.
    promotion: Piece,
}

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
    pub fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        Self { from, to, promotion: promotion.unwrap_or(Piece::Pawn) }
    }

    #[inline(always)]
    pub fn from(&self) -> Square { self.from }

    #[inline(always)]
    pub fn to(&self) -> Square { self.to }

    #[inline(always)]
    pub fn promotion(&self) -> Option<Piece> {
        (self.promotion != Piece::Pawn).then_some(self.promotion)
    }
}
