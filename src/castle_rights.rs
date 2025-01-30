use crate::square::File;

/// Represents a castling right for a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastleRights(u8);

impl Default for CastleRights {
    #[inline(always)]
    fn default() -> Self {
        Self(0b111_000_11)
    }
}

impl CastleRights {
    /// Get the default castle rights but without any rights.
    #[inline(always)]
    pub const fn default_no_rights() -> Self {
        Self(0b111_000_00)
    }

    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn set_ks_file(&mut self, f: File) {
        self.0 &= !0b111_000_00;
        self.0 |= (f as u8) << 5;
    }

    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn set_qs_file(&mut self, f: File) {
        self.0 &= !0b000_111_00;
        self.0 |= (f as u8) << 2;
    }

    /// Get the file of the king side rook.
    #[inline(always)]
    pub fn king_side_file(self) -> File {
        File::ALL[self.0 as usize >> 5]
    }

    /// Get the file of the queen side rook.
    #[inline(always)]
    pub fn queen_side_file(self) -> File {
        File::ALL[(self.0 as usize >> 2) & 7]
    }

    /// King-side rook have the right to castle if true.
    #[inline(always)]
    pub fn king_side(self) -> bool {
        self.0 & 2 != 0
    }

    /// Queen-side rook have the right to castle if true.
    #[inline(always)]
    pub fn queen_side(self) -> bool {
        self.0 & 1 != 0
    }

    /// The king is able to castle if true.
    #[inline(always)]
    pub fn any_side(self) -> bool {
        self.0 & 3 != 0
    }

    /// Disallow castling to the king-side rook.
    #[inline(always)]
    pub fn disallow_king_side(&mut self) {
        self.0 &= !2;
    }

    /// Disallow castling to the queen-side rook.
    #[inline(always)]
    pub fn disallow_queen_side(&mut self) {
        self.0 &= !1;
    }

    /// Disallow castling to any rook.
    #[inline(always)]
    pub fn disallow_castling(&mut self) {
        self.0 = 0;
    }

    /// Allow castling to the king-side rook.
    #[inline(always)]
    pub fn allow_king_side(&mut self) {
        self.0 |= 2;
    }

    /// Allow castling to the queen-side rook.
    #[inline(always)]
    pub fn allow_queen_side(&mut self) {
        self.0 |= 1;
    }
}
