use super::*;

impl Board {
    pub(crate) fn empty() -> Self {
        Self {
            pieces: [Bitboard::default(); 6],
            colors: [Bitboard::default(); 2],
            side_to_move: Color::White,

            mailbox: [0; 64],

            castle_rights: [CastleRights::default_no_rights(); 2],

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

    /// All of side to move's pawns on the board.
    #[inline(always)]
    pub fn our_pawns(&self) -> Bitboard { self.pawns_of(self.side_to_move()) }

    /// All of side to move's knights on the board.
    #[inline(always)]
    pub fn our_knights(&self) -> Bitboard { self.knights_of(self.side_to_move()) }

    /// All of side to move's bishops on the board.
    #[inline(always)]
    pub fn our_bishops(&self) -> Bitboard { self.bishops_of(self.side_to_move()) }

    /// All of side to move's rooks on the board.
    #[inline(always)]
    pub fn our_rooks(&self) -> Bitboard { self.rooks_of(self.side_to_move()) }

    /// All of side to move's queens on the board.
    #[inline(always)]
    pub fn our_queens(&self) -> Bitboard { self.queens_of(self.side_to_move()) }

    /// Side to move's king on the board.
    #[inline(always)]
    pub fn our_king(&self) -> Square { self.king_of(self.side_to_move()) }

    /// All of side to move's opponent's pawns on the board.
    #[inline(always)]
    pub fn their_pawns(&self) -> Bitboard { self.pawns_of(!self.side_to_move()) }

    /// All of side to move's opponent's knights on the board.
    #[inline(always)]
    pub fn their_knights(&self) -> Bitboard { self.knights_of(!self.side_to_move()) }

    /// All of side to move's opponent's bishops on the board.
    #[inline(always)]
    pub fn their_bishops(&self) -> Bitboard { self.bishops_of(!self.side_to_move()) }

    /// All of side to move's opponent's rooks on the board.
    #[inline(always)]
    pub fn their_rooks(&self) -> Bitboard { self.rooks_of(!self.side_to_move()) }

    /// All of side to move's opponent's queens on the board.
    #[inline(always)]
    pub fn their_queens(&self) -> Bitboard { self.queens_of(!self.side_to_move()) }

    /// Side to move's opponent's king on the board.
    #[inline(always)]
    pub fn their_king(&self) -> Square { self.king_of(!self.side_to_move()) }

    /// All of the pawns on the board that belongs to the side passed in.
    #[inline(always)]
    pub fn pawns_of(&self, color: Color) -> Bitboard { self.pawns() & self.color_combined(color) }

    /// All of the knights on the board that belongs to the side passed in.
    #[inline(always)]
    pub fn knights_of(&self, color: Color) -> Bitboard { self.knights() & self.color_combined(color) }

    /// All of the bishops on the board that belongs to the side passed in.
    #[inline(always)]
    pub fn bishops_of(&self, color: Color) -> Bitboard { self.bishops() & self.color_combined(color) }

    /// All of the rooks on the board that belongs to the side passed in.
    #[inline(always)]
    pub fn rooks_of(&self, color: Color) -> Bitboard { self.rooks() & self.color_combined(color) }

    /// All of the queens on the board that belongs to the side passed in.
    #[inline(always)]
    pub fn queens_of(&self, color: Color) -> Bitboard { self.queens() & self.color_combined(color) }

    /// The king on the board that belongs to the side passed in.
    #[inline(always)]
    pub fn king_of(&self, color: Color) -> Square {
        (self.kings() & self.color_combined(color))
            .try_into()
            .expect("there should only be a king for each side")
    }
}
