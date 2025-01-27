/// Represents a castling right for a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastleRights(u8);

impl Default for CastleRights {
    #[inline(always)]
    fn default() -> Self {
        Self(3)
    }
}

impl CastleRights {
    /// King-side rook (or the rook closer to the h-file if playing Chess960) have the right to
    /// castle if true.
    #[inline(always)]
    pub fn king_side(self) -> bool {
        self.0 & 1 != 0
    }

    /// Queen-side rook (or the rook closer to the a-file if playing Chess960) have the right to
    /// castle if true.
    #[inline(always)]
    pub fn queen_side(self) -> bool {
        self.0 & 2 != 0
    }

    /// The king is able to castle if true.
    #[inline(always)]
    pub fn any_side(self) -> bool {
        self.0 & 2 != 0
    }

    /// Disallow castling to the king-side rook.
    #[inline(always)]
    pub fn disallow_king_side(&mut self) {
        self.0 &= 3 - 1;
    }

    /// Disallow castling to the queen-side rook.
    #[inline(always)]
    pub fn disallow_queen_side(&mut self) {
        self.0 &= 3 - 2;
    }

    /// Disallow castling to any rook.
    #[inline(always)]
    pub fn disallow_castling(&mut self) {
        self.0 = 0;
    }

    /// Allow castling to the king-side rook.
    #[inline(always)]
    pub fn allow_king_side(&mut self) {
        self.0 |= 1;
    }

    /// Allow castling to the queen-side rook.
    #[inline(always)]
    pub fn allow_queen_side(&mut self) {
        self.0 |= 2;
    }
}
