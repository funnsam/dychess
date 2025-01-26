/// A chess board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastleRights(u8);

impl Default for CastleRights {
    fn default() -> Self {
        Self(3)
    }
}

impl CastleRights {
    /// King-side rook (or the rook closer to the h-file if playing Chess960) have the right to
    /// castle if true.
    pub fn king_side(self) -> bool {
        self.0 & 1 != 0
    }

    /// Queen-side rook (or the rook closer to the a-file if playing Chess960) have the right to
    /// castle if true.
    pub fn queen_side(self) -> bool {
        self.0 & 2 != 0
    }

    /// The king is able to castle if true.
    pub fn any_side(self) -> bool {
        self.0 & 2 != 0
    }

    /// Disallow castling to the king-side rook.
    pub fn disallow_king_side(&mut self) {
        self.0 &= 3 - 2;
    }

    /// Disallow castling to the queen-side rook.
    pub fn disallow_queen_side(&mut self) {
        self.0 &= 3 - 1;
    }

    /// Disallow castling to any rook.
    pub fn disallow_castling(&mut self) {
        self.0 = 0;
    }
}
