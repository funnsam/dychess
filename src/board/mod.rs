use crate::prelude::*;

pub mod epd;
pub mod movegen;
mod util;
mod zobrist;

/// A chess board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],
    side_to_move: Color,

    /// An internal square centric array for quicker piece and color lookup.
    mailbox: [u8; 64],

    /// Castle rights for white and black.
    castle_rights: [CastleRights; 2],

    hash: u64,
    en_passant: Option<File>,

    chess960: bool,
}

impl Default for Board {
    #[inline(always)]
    fn default() -> Self {
        Self::from_epd(
            false,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
        ).expect("valid position")
    }
}

impl Board {
    /// Make a move on the board.
    ///
    /// # Panics
    /// This function panics if the move's `from` square is empty.
    pub fn make_move(&mut self, mov: Move) {
        let move_bb = Bitboard::from(mov.from()) | mov.to().into();
        let (piece, _) = self.erase_piece(mov.from())
            .expect("tried to make invalid move: piece does not exist on move `from` square");
        let capture = self.place_piece(self.side_to_move(), mov.to(), mov.promotion().unwrap_or(piece));

        match piece {
            Piece::Pawn => if let Some(ep) = self.en_passant {
                if mov.from().file() != mov.to().file() && mov.to().file() == ep && !(pawn::ep_targets(self.side_to_move()) & mov.to().into()).is_empty() {
                    self.erase_piece(Square::new(ep, mov.from().rank()));
                }
            },
            Piece::Rook => {
                if mov.from() == Square::new(File::H, self.side_to_move().back_rank()) {
                    self.disallow_king_side_castle(self.side_to_move());
                } else if mov.from() == Square::new(File::A, self.side_to_move().back_rank()) {
                    self.disallow_queen_side_castle(self.side_to_move());
                }
            },
            Piece::King => {
                self.disallow_queen_side_castle(self.side_to_move());
                self.disallow_king_side_castle(self.side_to_move());

                if self.chess960 && capture == Some((Piece::Rook, self.side_to_move())) {
                    todo!("chess960 castling");
                } else if !self.chess960 && (move_bb & king::CASTLE_MOVE) == move_bb {
                    const ROOK_AT: [File; 8] = [
                        File::A,
                        File::A,
                        File::A,
                        File::A,
                        File::H,
                        File::H,
                        File::H,
                        File::H,
                    ];
                    const ROOK_TO: [File; 8] = [
                        File::D,
                        File::D,
                        File::D,
                        File::D,
                        File::F,
                        File::F,
                        File::F,
                        File::F,
                    ];

                    let rook_at = Square::new(ROOK_AT[mov.to().file() as usize], mov.to().rank());
                    let rook_to = Square::new(ROOK_TO[mov.to().file() as usize], mov.to().rank());
                    self.erase_piece(rook_at);
                    self.place_piece(self.side_to_move(), rook_to, Piece::Rook);
                }
            },
            _ => {},
        }

        if capture == Some((Piece::Rook, !self.side_to_move())) {
            if mov.to() == Square::new(File::H, (!self.side_to_move()).back_rank()) {
                self.disallow_king_side_castle(!self.side_to_move());
            } else if mov.to() == Square::new(File::A, (!self.side_to_move()).back_rank()) {
                self.disallow_queen_side_castle(!self.side_to_move());
            }
        }

        self.en_passant = (piece == Piece::Pawn && (move_bb & pawn::double_pushes(self.side_to_move())) == move_bb)
            .then_some(mov.from().file());
        self.side_to_move = !self.side_to_move();
        self.hash ^= zobrist::SIDE_TO_MOVE;
    }

    pub(crate) const fn allow_queen_side_castle(&mut self, color: Color) {
        if !self.castle_rights[color as usize].queen_side() {
            self.hash ^= zobrist::CASTLE[color as usize * 2 + 1];
        }
        self.castle_rights[color as usize].allow_queen_side();
    }

    pub(crate) const fn allow_king_side_castle(&mut self, color: Color) {
        if !self.castle_rights[color as usize].king_side() {
            self.hash ^= zobrist::CASTLE[color as usize * 2];
        }
        self.castle_rights[color as usize].allow_king_side();
    }

    pub(crate) const fn disallow_queen_side_castle(&mut self, color: Color) {
        if self.castle_rights[color as usize].queen_side() {
            self.hash ^= zobrist::CASTLE[color as usize * 2 + 1];
        }
        self.castle_rights[color as usize].disallow_queen_side();
    }

    pub(crate) const fn disallow_king_side_castle(&mut self, color: Color) {
        if self.castle_rights[color as usize].king_side() {
            self.hash ^= zobrist::CASTLE[color as usize * 2];
        }
        self.castle_rights[color as usize].disallow_king_side();
    }

    /// Get the hash of this position. This function is the same as the one used in the polyglot
    /// books, so that polyglot book lookup is very fast and easy.
    #[must_use]
    pub fn get_hash(&self) -> u64 {
        self.en_passant.map_or(self.hash, |f| {
            let r = pawn::double_push_to(!self.side_to_move());

            let p = self.pawns_of(self.side_to_move());
            let mut s = Bitboard::default();

            if let Some(p) = f.left(1) { s |= Bitboard::from(p) }
            if let Some(p) = f.right(1) { s |= Bitboard::from(p) }

            if (r & s & p).is_empty() {
                self.hash
            } else {
                self.hash ^ zobrist::EP_FILE[f as usize]
            }
        })
    }

    /// Pass this move to the side to move.
    ///
    /// # Notes
    /// This should only be called if we aren't in check.
    ///
    /// # Returns
    /// This returns a [`BoardUnpasser`] to undo this passing move by calling
    /// [`Self::restore_passed`].
    #[inline(always)]
    pub fn null_move(&mut self) -> NullMoveRestorer {
        self.side_to_move = !self.side_to_move();
        self.hash ^= zobrist::SIDE_TO_MOVE;

        NullMoveRestorer {
            en_passant: core::mem::take(&mut self.en_passant),
        }
    }

    /// Restore a passed move that was made by [`Self::pass_move`].
    #[inline(always)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn restore_null_move(&mut self, restorer: NullMoveRestorer) {
        self.side_to_move = !self.side_to_move();
        self.hash ^= zobrist::SIDE_TO_MOVE;

        self.en_passant = restorer.en_passant;
    }

    #[inline(always)]
    fn place_piece(&mut self, color: Color, square: Square, piece: Piece) -> Option<(Piece, Color)> {
        let ret = self.erase_piece(square);
        self.place_unchecked(color, square, piece);
        ret
    }

    #[inline(always)]
    fn place_unchecked(&mut self, color: Color, square: Square, piece: Piece) {
        let to_bb = Bitboard::from(square);
        self.pieces[piece as usize] |= to_bb;
        self.colors[color as usize] |= to_bb;
        self.mailbox[square.to_usize()] = mailbox_element(color, piece);
        self.hash ^= zobrist::piece(piece, color, square);
    }

    #[inline(always)]
    fn erase_piece(&mut self, square: Square) -> Option<(Piece, Color)> {
        let piece = self.piece_and_color_on(square);

        if let Some((piece, color)) = piece {
            let bb = Bitboard::from(square);

            self.pieces[piece as usize] ^= bb;
            self.colors[color as usize] ^= bb;
            self.mailbox[square.to_usize()] = 0;
            self.hash ^= zobrist::piece(piece, color, square);
        }

        piece
    }

    pub(crate) fn piece_targets<const ATKDEF: bool>(&self, color: Color, piece: Piece, sq: Square) -> Bitboard {
        let bb = match piece {
            Piece::Pawn => {
                let captures = pawn::captures(color, sq);

                if ATKDEF {
                    captures
                } else {
                    let advances = pawn::advances(color, sq, self.combined());

                    advances | (captures & (self.color_combined(!color) | self.ep_square(color)))
                }
            },
            Piece::Knight => knight::moves(sq),
            Piece::Bishop => bishop::moves(sq, self.combined()),
            Piece::Rook => rook::moves(sq, self.combined()),
            Piece::Queen => queen::moves(sq, self.combined()),
            Piece::King => {
                let mut moves = king::moves(sq);

                if !ATKDEF && self.castle_rights_of(color).king_side() {
                    if let Some(ks_rook) = (self.rooks_of(color) & self.castle_rights_of(color).king_side_file().into() & color.back_rank().into()).first_square() {
                        if (king::castle_clearance(color, sq.file(), ks_rook.file()) & self.combined()).is_empty()
                            && (king::castle_path(color, sq.file(), ks_rook.file()) & self.side_attack_def(!color)).is_empty()
                        {
                            moves |= Bitboard::from(if self.chess960 { ks_rook} else { Square::new(File::G, sq.rank()) });
                        }
                    }
                }

                if !ATKDEF && self.castle_rights_of(color).queen_side() {
                    if let Some(qs_rook) = (self.rooks_of(color) & self.castle_rights_of(color).queen_side_file().into() & color.back_rank().into()).first_square() {
                        if (king::castle_clearance(color, sq.file(), qs_rook.file()) & self.combined()).is_empty()
                            && (king::castle_path(color, sq.file(), qs_rook.file()) & self.side_attack_def(!color)).is_empty()
                        {
                            moves |= Bitboard::from(if self.chess960 { qs_rook } else { Square::new(File::C, sq.rank()) });
                        }
                    }
                }

                moves
            },
        };

        if ATKDEF {
            bb
        } else {
            bb & !self.color_combined(self.side_to_move())
        }
    }

    #[inline(always)]
    fn ep_square(&self, color: Color) -> Bitboard {
        self.en_passant.map_or_else(Bitboard::default,
            |f| Square::new(f, [Rank::_3, Rank::_6][!color as usize]).into()
        )
    }

    /// Get if the side to move is in check.
    ///
    /// # Example
    /// ```
    /// # use dychess::prelude::*;
    /// #
    /// let board = Board::from_epd(false, "8/4k3/8/8/8/8/3K4/8 w - -").unwrap();
    /// assert!(!board.is_check());
    ///
    /// let board = Board::from_epd(false, "8/4k3/8/8/8/2p5/3K4/8 w - -").unwrap();
    /// assert!(board.is_check());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn is_check(&self) -> bool {
        self.is_side_check(self.side_to_move())
    }

    /// Get if the side just moved is in check. This will indicate if the pseudo-legal move
    /// previously moved is actually legal.
    #[inline(always)]
    #[must_use]
    pub fn is_illegal(&self) -> bool {
        self.is_side_check(!self.side_to_move())
    }

    #[must_use]
    fn is_side_check(&self, color: Color) -> bool {
        let combined = self.combined();
        let ksq = self.king_of(color);

        !(knight::moves(ksq) & self.knights_of(!color)).is_empty()
            || !(pawn::captures(color, ksq) & self.pawns_of(!color)).is_empty()
            || !(king::moves(ksq) & self.kings()).is_empty()
            || !(bishop::moves(ksq, combined) & (self.bishops_of(!color) | self.queens_of(!color))).is_empty()
            || !(rook::moves(ksq, combined) & (self.rooks_of(!color) | self.queens_of(!color))).is_empty()
    }

    #[must_use]
    pub fn side_attack_def(&self, color: Color) -> Bitboard {
        let mut atkdef = Bitboard::default();
        for sq in self.color_combined(color) {
            // SAFETY: we're only iterating through squares with pieces
            let piece = unsafe { self.piece_on(sq).unwrap_unchecked() };
            atkdef |= self.piece_targets::<true>(color, piece, sq);
        }
        atkdef
    }

    /// Get the attackers and defenders of a particular square.
    #[must_use]
    pub fn attackers(&self, sq: Square) -> Bitboard {
        let combined = self.combined();

        pawn::captures(!Color::White, sq) & self.white_pawns()
            | pawn::captures(!Color::Black, sq) & self.black_pawns()
            | knight::moves(sq) & self.knights()
            | bishop::moves(sq, combined) & (self.bishops() | self.queens())
            | rook::moves(sq, combined) & (self.rooks() | self.queens())
            | king::moves(sq) & self.kings()
    }

    // #[doc(hidden)]
    // pub fn _check_legality(&self) {
    //     assert_eq!(self.pieces.into_iter().fold(Bitboard::default(), |a, p| {
    //         if !(a & p).is_empty() {
    //             panic!("piece table overlap");
    //         }

    //         a ^ p
    //     }), self.combined(), "piece tb cumul OR != color tb cumul OR");

    //     for sq in self.combined() {
    //         if let None = self.piece_on(sq) {
    //             panic!("{self:?}\n{sq}");
    //         }
    //     }
    // }
}

#[inline(always)]
const fn mailbox_element(color: Color, piece: Piece) -> u8 {
    ((color as u8 + 1) << 3) | (piece as u8)
}

impl core::hash::Hash for Board {
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

#[derive(Debug)]
pub struct NullMoveRestorer {
    en_passant: Option<File>,
}
