use super::*;

impl Board {
    pub(crate) fn empty() -> Self {
        Self {
            pieces: [Bitboard::default(); 6],
            colors: [Bitboard::default(); 2],
            side_to_move: Color::White,

            mailbox: [0; 64],

            castle_rights: [CastleRights::default(); 2],

            hash: 0,
            en_passant: None,

            chess960: false,
        }
    }

    /// Get the side to move.
    #[inline(always)]
    pub fn side_to_move(&self) -> Color { self.side_to_move }

    /// Get the piece and color on a given square.
    #[inline(always)]
    pub fn piece_and_color_on(&self, square: Square) -> Option<(Piece, Color)> {
        let element = self.mailbox[square.to_usize()];
        (element != 0).then(|| {
            (Piece::ALL[element as usize & 7], Color::ALL[(element as usize >> 3) - 1])
        })
    }

    /// Get the piece on a given square.
    #[inline(always)]
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        self.piece_and_color_on(square).map(|(p, _)| p)
    }

    /// Get the color of the piece on a given square.
    #[inline(always)]
    pub fn color_on(&self, square: Square) -> Option<Color> {
        self.piece_and_color_on(square).map(|(_, c)| c)
    }

    /// All pieces combined in a bitboard.
    #[inline(always)]
    pub fn combined(&self) -> Bitboard { self.colors[0] | self.colors[1] }

    /// All pieces that belongs to the specified color.
    #[inline(always)]
    pub fn color_combined(&self, color: Color) -> Bitboard { self.colors[color as usize] }

    /// All pieces that belongs to white.
    #[inline(always)]
    pub fn white_pieces(&self) -> Bitboard { self.color_combined(Color::White) }

    /// All pieces that belongs to black.
    #[inline(always)]
    pub fn black_pieces(&self) -> Bitboard { self.color_combined(Color::Black) }

    /// All pieces that belongs to the side to move.
    #[inline(always)]
    pub fn our_pieces(&self) -> Bitboard { self.color_combined(self.side_to_move()) }

    /// All pieces that belongs to the side not to move.
    #[inline(always)]
    pub fn their_pieces(&self) -> Bitboard { self.color_combined(!self.side_to_move()) }

    /// All squares with the specified piece.
    #[inline(always)]
    pub fn piece_combined(&self, piece: Piece) -> Bitboard { self.pieces[piece as usize] }

    /// All pawns on the board.
    #[inline(always)]
    pub fn pawns(&self) -> Bitboard { self.piece_combined(Piece::Pawn) }

    /// All knights on the board.
    #[inline(always)]
    pub fn knights(&self) -> Bitboard { self.piece_combined(Piece::Knight) }

    /// All bishops on the board.
    #[inline(always)]
    pub fn bishops(&self) -> Bitboard { self.piece_combined(Piece::Bishop) }

    /// All rooks on the board.
    #[inline(always)]
    pub fn rooks(&self) -> Bitboard { self.piece_combined(Piece::Rook) }

    /// All queens on the board.
    #[inline(always)]
    pub fn queens(&self) -> Bitboard { self.piece_combined(Piece::Queen) }

    /// All kings on the board.
    #[inline(always)]
    pub fn kings(&self) -> Bitboard { self.piece_combined(Piece::King) }

    /// All of our pawns on the board.
    #[inline(always)]
    pub fn our_pawns(&self) -> Bitboard { self.piece_combined(Piece::Pawn) & self.our_pieces() }

    /// All of our knights on the board.
    #[inline(always)]
    pub fn our_knights(&self) -> Bitboard { self.piece_combined(Piece::Knight) & self.our_pieces() }

    /// All of our bishops on the board.
    #[inline(always)]
    pub fn our_bishops(&self) -> Bitboard { self.piece_combined(Piece::Bishop) & self.our_pieces() }

    /// All of our rooks on the board.
    #[inline(always)]
    pub fn our_rooks(&self) -> Bitboard { self.piece_combined(Piece::Rook) & self.our_pieces() }

    /// All of our queens on the board.
    #[inline(always)]
    pub fn our_queens(&self) -> Bitboard { self.piece_combined(Piece::Queen) & self.our_pieces() }

    /// Our king on the board.
    #[inline(always)]
    pub fn our_king(&self) -> Square {
        (self.piece_combined(Piece::King) & self.our_pieces())
            .try_into()
            .expect("there should only be a king for each side")
    }
}
