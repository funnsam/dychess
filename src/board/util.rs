use super::{Bitboard, Board, CastleRights, Color, Piece, Square};

impl Board {
    #[must_use]
    pub(crate) fn empty() -> Self {
        Self {
            pieces: [Bitboard::default(); 6],
            colors: [Bitboard::default(); 2],
            side_to_move: Color::Black,

            mailbox: [0; 64],

            castle_rights: [CastleRights::default_no_rights(); 2],

            hash: 0,
            en_passant: None,

            chess960: false,
        }
    }

    /// Get the side to move.
    #[inline(always)]
    #[must_use]
    pub const fn side_to_move(&self) -> Color { self.side_to_move }

    /// Get the piece and color on a given square.
    #[inline(always)]
    #[must_use]
    pub fn piece_and_color_on(&self, square: Square) -> Option<(Piece, Color)> {
        let element = unsafe { *self.mailbox.get_unchecked(square.to_usize()) };

        (element != 0).then(|| unsafe {
            (
                Piece::from_index_unchecked(element & 7),
                Color::from_index_unchecked((element >> 3) - 1),
            )
        })
    }

    /// Get the piece on a given square.
    #[inline(always)]
    #[must_use]
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        self.piece_and_color_on(square).map(|(p, _)| p)
    }

    /// Get the color of the piece on a given square.
    #[inline(always)]
    #[must_use]
    pub fn color_on(&self, square: Square) -> Option<Color> {
        self.piece_and_color_on(square).map(|(_, c)| c)
    }

    /// All pieces combined in a bitboard.
    #[inline(always)]
    #[must_use]
    pub const fn combined(&self) -> Bitboard { Bitboard(self.colors[0].0 | self.colors[1].0) }

    /// All pieces that belongs to the specified color.
    #[inline(always)]
    #[must_use]
    pub const fn color_combined(&self, color: Color) -> Bitboard { self.colors[color as usize] }

    /// All pieces that belongs to white.
    #[inline(always)]
    #[must_use]
    pub const fn white_pieces(&self) -> Bitboard { self.color_combined(Color::White) }

    /// All pieces that belongs to black.
    #[inline(always)]
    #[must_use]
    pub const fn black_pieces(&self) -> Bitboard { self.color_combined(Color::Black) }

    /// All pieces that belongs to the side to move.
    #[inline(always)]
    #[must_use]
    pub const fn our_pieces(&self) -> Bitboard { self.color_combined(self.side_to_move()) }

    /// All pieces that belongs to the side not to move.
    #[inline(always)]
    #[must_use]
    pub fn their_pieces(&self) -> Bitboard { self.color_combined(!self.side_to_move()) }

    /// All squares with the specified piece.
    #[inline(always)]
    #[must_use]
    pub const fn piece_combined(&self, piece: Piece) -> Bitboard { self.pieces[piece as usize] }

    /// All pawns on the board.
    #[inline(always)]
    #[must_use]
    pub const fn pawns(&self) -> Bitboard { self.piece_combined(Piece::Pawn) }

    /// All knights on the board.
    #[inline(always)]
    #[must_use]
    pub const fn knights(&self) -> Bitboard { self.piece_combined(Piece::Knight) }

    /// All bishops on the board.
    #[inline(always)]
    #[must_use]
    pub const fn bishops(&self) -> Bitboard { self.piece_combined(Piece::Bishop) }

    /// All rooks on the board.
    #[inline(always)]
    #[must_use]
    pub const fn rooks(&self) -> Bitboard { self.piece_combined(Piece::Rook) }

    /// All queens on the board.
    #[inline(always)]
    #[must_use]
    pub const fn queens(&self) -> Bitboard { self.piece_combined(Piece::Queen) }

    /// All kings on the board.
    #[inline(always)]
    #[must_use]
    pub const fn kings(&self) -> Bitboard { self.piece_combined(Piece::King) }

    /// All of side to move's pawns on the board.
    #[inline(always)]
    #[must_use]
    pub fn our_pawns(&self) -> Bitboard { self.pawns_of(self.side_to_move()) }

    /// All of side to move's knights on the board.
    #[inline(always)]
    #[must_use]
    pub fn our_knights(&self) -> Bitboard { self.knights_of(self.side_to_move()) }

    /// All of side to move's bishops on the board.
    #[inline(always)]
    #[must_use]
    pub fn our_bishops(&self) -> Bitboard { self.bishops_of(self.side_to_move()) }

    /// All of side to move's rooks on the board.
    #[inline(always)]
    #[must_use]
    pub fn our_rooks(&self) -> Bitboard { self.rooks_of(self.side_to_move()) }

    /// All of side to move's queens on the board.
    #[inline(always)]
    #[must_use]
    pub fn our_queens(&self) -> Bitboard { self.queens_of(self.side_to_move()) }

    /// Side to move's king on the board.
    #[inline(always)]
    #[must_use]
    pub fn our_king(&self) -> Square { self.king_of(self.side_to_move()) }

    /// All of side to move's opponent's pawns on the board.
    #[inline(always)]
    #[must_use]
    pub fn their_pawns(&self) -> Bitboard { self.pawns_of(!self.side_to_move()) }

    /// All of side to move's opponent's knights on the board.
    #[inline(always)]
    #[must_use]
    pub fn their_knights(&self) -> Bitboard { self.knights_of(!self.side_to_move()) }

    /// All of side to move's opponent's bishops on the board.
    #[inline(always)]
    #[must_use]
    pub fn their_bishops(&self) -> Bitboard { self.bishops_of(!self.side_to_move()) }

    /// All of side to move's opponent's rooks on the board.
    #[inline(always)]
    #[must_use]
    pub fn their_rooks(&self) -> Bitboard { self.rooks_of(!self.side_to_move()) }

    /// All of side to move's opponent's queens on the board.
    #[inline(always)]
    #[must_use]
    pub fn their_queens(&self) -> Bitboard { self.queens_of(!self.side_to_move()) }

    /// Side to move's opponent's king on the board.
    #[inline(always)]
    #[must_use]
    pub fn their_king(&self) -> Square { self.king_of(!self.side_to_move()) }

    /// All of white's pawns on the board.
    #[inline(always)]
    #[must_use]
    pub fn white_pawns(&self) -> Bitboard { self.pawns_of(Color::White) }

    /// All of white's knights on the board.
    #[inline(always)]
    #[must_use]
    pub fn white_knights(&self) -> Bitboard { self.knights_of(Color::White) }

    /// All of white's bishops on the board.
    #[inline(always)]
    #[must_use]
    pub fn white_bishops(&self) -> Bitboard { self.bishops_of(Color::White) }

    /// All of white's rooks on the board.
    #[inline(always)]
    #[must_use]
    pub fn white_rooks(&self) -> Bitboard { self.rooks_of(Color::White) }

    /// All of white's queens on the board.
    #[inline(always)]
    #[must_use]
    pub fn white_queens(&self) -> Bitboard { self.queens_of(Color::White) }

    /// White's king on the board.
    #[inline(always)]
    #[must_use]
    pub fn white_king(&self) -> Square { self.king_of(Color::White) }

    /// All of black's pawns on the board.
    #[inline(always)]
    #[must_use]
    pub fn black_pawns(&self) -> Bitboard { self.pawns_of(Color::Black) }

    /// All of black's knights on the board.
    #[inline(always)]
    #[must_use]
    pub fn black_knights(&self) -> Bitboard { self.knights_of(Color::Black) }

    /// All of black's bishops on the board.
    #[inline(always)]
    #[must_use]
    pub fn black_bishops(&self) -> Bitboard { self.bishops_of(Color::Black) }

    /// All of black's rooks on the board.
    #[inline(always)]
    #[must_use]
    pub fn black_rooks(&self) -> Bitboard { self.rooks_of(Color::Black) }

    /// All of black's queens on the board.
    #[inline(always)]
    #[must_use]
    pub fn black_queens(&self) -> Bitboard { self.queens_of(Color::Black) }

    /// Black's king on the board.
    #[inline(always)]
    #[must_use]
    pub fn black_king(&self) -> Square { self.king_of(Color::Black) }

    /// All of the pawns on the board that belongs to the side passed in.
    #[inline(always)]
    #[must_use]
    pub fn pawns_of(&self, color: Color) -> Bitboard { self.pawns() & self.color_combined(color) }

    /// All of the knights on the board that belongs to the side passed in.
    #[inline(always)]
    #[must_use]
    pub fn knights_of(&self, color: Color) -> Bitboard { self.knights() & self.color_combined(color) }

    /// All of the bishops on the board that belongs to the side passed in.
    #[inline(always)]
    #[must_use]
    pub fn bishops_of(&self, color: Color) -> Bitboard { self.bishops() & self.color_combined(color) }

    /// All of the rooks on the board that belongs to the side passed in.
    #[inline(always)]
    #[must_use]
    pub fn rooks_of(&self, color: Color) -> Bitboard { self.rooks() & self.color_combined(color) }

    /// All of the queens on the board that belongs to the side passed in.
    #[inline(always)]
    #[must_use]
    pub fn queens_of(&self, color: Color) -> Bitboard { self.queens() & self.color_combined(color) }

    /// The king on the board that belongs to the side passed in.
    ///
    /// # Panics
    /// This function panics if there are more than 1 kings of the given side or there are none.
    #[inline(always)]
    #[must_use]
    pub fn king_of(&self, color: Color) -> Square {
        (self.kings() & self.color_combined(color))
            .try_into()
            .expect("there should only be a king for each side")
    }

    /// Get the castle rights of a side.
    #[inline(always)]
    #[must_use]
    pub const fn castle_rights_of(&self, color: Color) -> CastleRights { self.castle_rights[color as usize] }

    /// Get the castle rights of white.
    #[inline(always)]
    #[must_use]
    pub const fn white_castle_rights(&self) -> CastleRights { self.castle_rights_of(Color::White) }

    /// Get the castle rights of black.
    #[inline(always)]
    #[must_use]
    pub const fn black_castle_rights(&self) -> CastleRights { self.castle_rights_of(Color::Black) }

    /// Get the castle rights of the side to move.
    #[inline(always)]
    #[must_use]
    pub const fn our_castle_rights(&self) -> CastleRights { self.castle_rights_of(self.side_to_move()) }

    /// Get the castle rights of the side to move's opponent.
    #[inline(always)]
    #[must_use]
    pub fn their_castle_rights(&self) -> CastleRights { self.castle_rights_of(!self.side_to_move()) }
}
