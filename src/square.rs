use core::fmt;

use crate::color::Color;

/// A square on the board.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    /// Make a new square from a file and rank.
    #[inline(always)]
    pub const fn new(file: File, rank: Rank) -> Self {
        Self(((rank as u8) << 3) | (file as u8))
    }

    /// Make a new square from an index.
    ///
    /// # Panics
    /// It will panic if `idx > 63`.
    #[inline(always)]
    pub const fn from_index(idx: u8) -> Self {
        assert!(idx < 64);
        Self(idx)
    }

    /// Get the square in the point of view of the given color.
    #[inline(always)]
    pub const fn pov(self, color: Color) -> Self {
        match color {
            Color::White => self,
            Color::Black => self.black_pov(),
        }
    }

    /// Get the square in the point of view of black.
    #[inline(always)]
    pub const fn black_pov(self) -> Self {
        Self(self.0 ^ 0b111_000)
    }

    /// Converts this square to an `u8`.
    #[inline(always)]
    pub const fn to_u8(self) -> u8 { self.0 as _ }

    /// Converts this square to an `usize`.
    #[inline(always)]
    pub const fn to_usize(self) -> usize { self.0 as _ }

    /// The file of this square.
    #[inline(always)]
    pub const fn file(self) -> File {
        File::ALL[self.0 as usize & 7]
    }

    /// The rank of this square.
    #[inline(always)]
    pub const fn rank(self) -> Rank {
        Rank::ALL[(self.0 >> 3) as usize]
    }
}

/// A file (or column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A, B, C, D, E, F, G, H
}

/// A rank (or row).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    _1, _2, _3, _4, _5, _6, _7, _8
}

impl File {
    pub const ALL: [Self; 8] = [
        Self::A, Self::B, Self::C, Self::D, Self::E, Self::F, Self::G, Self::H
    ];

    /// Get the file `n` files to the right, wrapping around if it will overflow.
    #[inline(always)]
    pub const fn right_wrap(self, n: usize) -> Self {
        Self::ALL[(self as usize + n) % 8]
    }

    /// Get the file `n` files to the right;
    #[inline(always)]
    pub const fn right(self, n: usize) -> Option<Self> {
        let idx = self as usize + n;
        if idx >= 8 {
            return None;
        }

        Some(Self::ALL[idx])
    }

    /// Get the file `n` files to the left, wrapping around if it will underflow.
    #[inline(always)]
    pub const fn left_wrap(self, n: usize) -> Self {
        Self::ALL[(self as usize + 8 - n % 8) % 8]
    }

    /// Get the file `n` files to the left.
    #[inline(always)]
    pub const fn left(self, n: usize) -> Option<Self> {
        if (self as usize) < n {
            return None;
        }

        Some(Self::ALL[self as usize - n])
    }
}

impl Rank {
    pub const ALL: [Self; 8] = [
        Self::_1, Self::_2, Self::_3, Self::_4, Self::_5, Self::_6, Self::_7, Self::_8
    ];

    /// Calls [Self::invert] if `color` is black, otherwise returns `self`.
    #[inline(always)]
    pub const fn invert_if_black(self, color: Color) -> Self {
        match color {
            Color::White => self,
            Color::Black => self.invert(),
        }
    }

    /// Invert the rank.
    #[inline(always)]
    pub const fn invert(self) -> Self {
        Self::ALL[self as usize ^ 7]
    }

    /// Get the rank `n` ranks up, wrapping around if it will overflow.
    #[inline(always)]
    pub const fn up_wrap(self, n: usize) -> Self {
        Self::ALL[(self as usize + n) % 8]
    }

    /// Get the rank `n` ranks up.
    #[inline(always)]
    pub const fn up(self, n: usize) -> Option<Self> {
        let idx = self as usize + n;
        if idx >= 8 {
            return None;
        }

        Some(Self::ALL[idx])
    }

    /// Get the rank `n` ranks down, wrapping around if it will underflow.
    #[inline(always)]
    pub const fn down_wrap(self, n: usize) -> Self {
        Self::ALL[(self as usize + 8 - n % 8) % 8]
    }

    /// Get the rank `n` ranks down.
    #[inline(always)]
    pub const fn down(self, n: usize) -> Option<Self> {
        if (self as usize) < n {
            return None;
        }

        Some(Self::ALL[self as usize - n])
    }

    /// Get the rank forward and wrap around if overflowed.
    #[inline(always)]
    pub const fn forward_wrap(self, color: Color, n: usize) -> Self {
        match color {
            Color::White => self.up_wrap(n),
            Color::Black => self.down_wrap(n),
        }
    }

    /// Get the rank forward.
    #[inline(always)]
    pub const fn forward(self, color: Color, n: usize) -> Option<Self> {
        match color {
            Color::White => self.up(n),
            Color::Black => self.down(n),
        }
    }

    /// Get the rank backward and wrap around if overflowed.
    #[inline(always)]
    pub const fn backward_wrap(self, color: Color, n: usize) -> Self {
        match color {
            Color::White => self.down_wrap(n),
            Color::Black => self.up_wrap(n),
        }
    }

    /// Get the rank backward.
    #[inline(always)]
    pub const fn backward(self, color: Color, n: usize) -> Option<Self> {
        match color {
            Color::White => self.down(n),
            Color::Black => self.up(n),
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (b'a' + *self as u8) as char)
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8 + 1)
    }
}

impl Square {
    pub const A1: Self = Self::new(File::A, Rank::_1);
    pub const A2: Self = Self::new(File::A, Rank::_2);
    pub const A3: Self = Self::new(File::A, Rank::_3);
    pub const A4: Self = Self::new(File::A, Rank::_4);
    pub const A5: Self = Self::new(File::A, Rank::_5);
    pub const A6: Self = Self::new(File::A, Rank::_6);
    pub const A7: Self = Self::new(File::A, Rank::_7);
    pub const A8: Self = Self::new(File::A, Rank::_8);

    pub const B1: Self = Self::new(File::B, Rank::_1);
    pub const B2: Self = Self::new(File::B, Rank::_2);
    pub const B3: Self = Self::new(File::B, Rank::_3);
    pub const B4: Self = Self::new(File::B, Rank::_4);
    pub const B5: Self = Self::new(File::B, Rank::_5);
    pub const B6: Self = Self::new(File::B, Rank::_6);
    pub const B7: Self = Self::new(File::B, Rank::_7);
    pub const B8: Self = Self::new(File::B, Rank::_8);

    pub const C1: Self = Self::new(File::C, Rank::_1);
    pub const C2: Self = Self::new(File::C, Rank::_2);
    pub const C3: Self = Self::new(File::C, Rank::_3);
    pub const C4: Self = Self::new(File::C, Rank::_4);
    pub const C5: Self = Self::new(File::C, Rank::_5);
    pub const C6: Self = Self::new(File::C, Rank::_6);
    pub const C7: Self = Self::new(File::C, Rank::_7);
    pub const C8: Self = Self::new(File::C, Rank::_8);

    pub const D1: Self = Self::new(File::D, Rank::_1);
    pub const D2: Self = Self::new(File::D, Rank::_2);
    pub const D3: Self = Self::new(File::D, Rank::_3);
    pub const D4: Self = Self::new(File::D, Rank::_4);
    pub const D5: Self = Self::new(File::D, Rank::_5);
    pub const D6: Self = Self::new(File::D, Rank::_6);
    pub const D7: Self = Self::new(File::D, Rank::_7);
    pub const D8: Self = Self::new(File::D, Rank::_8);

    pub const E1: Self = Self::new(File::E, Rank::_1);
    pub const E2: Self = Self::new(File::E, Rank::_2);
    pub const E3: Self = Self::new(File::E, Rank::_3);
    pub const E4: Self = Self::new(File::E, Rank::_4);
    pub const E5: Self = Self::new(File::E, Rank::_5);
    pub const E6: Self = Self::new(File::E, Rank::_6);
    pub const E7: Self = Self::new(File::E, Rank::_7);
    pub const E8: Self = Self::new(File::E, Rank::_8);

    pub const F1: Self = Self::new(File::F, Rank::_1);
    pub const F2: Self = Self::new(File::F, Rank::_2);
    pub const F3: Self = Self::new(File::F, Rank::_3);
    pub const F4: Self = Self::new(File::F, Rank::_4);
    pub const F5: Self = Self::new(File::F, Rank::_5);
    pub const F6: Self = Self::new(File::F, Rank::_6);
    pub const F7: Self = Self::new(File::F, Rank::_7);
    pub const F8: Self = Self::new(File::F, Rank::_8);

    pub const G1: Self = Self::new(File::G, Rank::_1);
    pub const G2: Self = Self::new(File::G, Rank::_2);
    pub const G3: Self = Self::new(File::G, Rank::_3);
    pub const G4: Self = Self::new(File::G, Rank::_4);
    pub const G5: Self = Self::new(File::G, Rank::_5);
    pub const G6: Self = Self::new(File::G, Rank::_6);
    pub const G7: Self = Self::new(File::G, Rank::_7);
    pub const G8: Self = Self::new(File::G, Rank::_8);

    pub const H1: Self = Self::new(File::H, Rank::_1);
    pub const H2: Self = Self::new(File::H, Rank::_2);
    pub const H3: Self = Self::new(File::H, Rank::_3);
    pub const H4: Self = Self::new(File::H, Rank::_4);
    pub const H5: Self = Self::new(File::H, Rank::_5);
    pub const H6: Self = Self::new(File::H, Rank::_6);
    pub const H7: Self = Self::new(File::H, Rank::_7);
    pub const H8: Self = Self::new(File::H, Rank::_8);

    pub const ALL: [Self; 64] = [
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
    ];
}
