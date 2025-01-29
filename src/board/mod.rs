use crate::prelude::*;

pub mod fen;
pub mod movegen;
pub mod util;

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
    fn default() -> Self {
        Board::from_fen(
            false,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ).expect("valid position")
    }
}

impl Board {
    pub fn copy_make_move(&self, mov: Move) -> Self {
        let mut after = self.clone();
        after.make_move(mov);
        after
    }

    pub fn make_move(&mut self, mov: Move) {
        let move_bb = Bitboard::from(mov.from()) | mov.to().into();
        let (piece, _) = self.erase_piece(mov.from())
            .expect("tried to make invalid move: piece does not exist on move `from` square");
        let capture = self.place_piece(self.side_to_move(), mov.to(), piece);

        match piece {
            Piece::Pawn => if let Some(ep) = self.en_passant {
                if mov.from().file() != mov.to().file() && mov.to().file() == ep && !(pawn::EP_TARGETS & mov.to().into()).is_empty() {
                    self.erase_piece(Square::new(ep, mov.from().rank()));
                }
            },
            Piece::Rook => {
                if (mov.from() == Square::A1 && self.side_to_move() == Color::White) ||
                    (mov.from() == Square::A8 && self.side_to_move() == Color::Black)
                {
                    self.castle_rights[self.side_to_move() as usize].disallow_queen_side()
                } else if (mov.from() == Square::H1 && self.side_to_move() == Color::White) ||
                    (mov.from() == Square::H8 && self.side_to_move() == Color::Black)
                {
                    self.castle_rights[self.side_to_move() as usize].disallow_king_side()
                }
            },
            Piece::King => {
                self.castle_rights[self.side_to_move() as usize].disallow_castling();

                if self.chess960 && capture == Some((Piece::Rook, self.side_to_move())) {
                    todo!("chess960 castling");
                }

                if !self.chess960 && (move_bb & king::CASTLE_MOVE) == move_bb {
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

        self.en_passant = (piece == Piece::Pawn && (move_bb & pawn::double_pushes(self.side_to_move())) == move_bb)
            .then_some(mov.from().file());
        self.side_to_move = !self.side_to_move();
    }

    fn place_piece(&mut self, color: Color, square: Square, piece: Piece) -> Option<(Piece, Color)> {
        let ret = self.erase_piece(square);
        self.place_unchecked(color, square, piece);
        ret
    }

    fn place_unchecked(&mut self, color: Color, square: Square, piece: Piece) {
        let to_bb = Bitboard::from(square);
        self.pieces[piece as usize] |= to_bb;
        self.colors[color as usize] |= to_bb;
        self.mailbox[square.to_usize()] = mailbox_element(color, piece);
        // TODO: update hash
    }

    fn erase_piece(&mut self, square: Square) -> Option<(Piece, Color)> {
        let piece = self.piece_and_color_on(square);

        if let Some((piece, color)) = piece {
            let bb = Bitboard::from(square);
            self.pieces[piece as usize] &= !bb;
            self.colors[color as usize] &= !bb;
            self.mailbox[square.to_usize()] = 0;
            // TODO: update hash
        }

        piece
    }

    pub(crate) fn piece_targets(&self, castle: bool, color: Color, piece: Piece, sq: Square) -> Bitboard {
        match piece {
            Piece::Pawn => {
                let advances = pawn::advances(color, sq, self.combined());
                let captures = pawn::captures(color, sq);
                advances | (captures & (self.color_combined(!color) | self.ep_square(color)))
            },
            Piece::Knight => knight::moves(sq),
            Piece::Bishop => bishop::moves(sq, self.combined()),
            Piece::Rook => rook::moves(sq, self.combined()),
            Piece::Queen => queen::moves(sq, self.combined()),
            Piece::King => {
                let mut moves = king::moves(sq);

                if castle && self.castle_rights[color as usize].king_side() {
                    if let Some(ks_rook) = (self.rooks_of(color) & self.castle_rights[color as usize].king_side_file().into() & color.back_rank().into()).first_square() {
                        if (king::castle_clearance(color, sq.file(), ks_rook.file()) & self.combined()).is_empty()
                            && (king::castle_path(color, sq.file(), ks_rook.file()) & self.side_attack_def(!color)).is_empty()
                        {
                            moves |= Bitboard::from(if !self.chess960 { Square::new(File::G, sq.rank()) } else { ks_rook })
                        }
                    }
                }

                if castle && self.castle_rights[color as usize].queen_side() {
                    if let Some(qs_rook) = (self.rooks_of(color) & self.castle_rights[color as usize].queen_side_file().into() & color.back_rank().into()).first_square() {
                        if (king::castle_clearance(color, sq.file(), qs_rook.file()) & self.combined()).is_empty()
                            && (king::castle_path(color, sq.file(), qs_rook.file()) & self.side_attack_def(!color)).is_empty()
                        {
                            moves |= Bitboard::from(if !self.chess960 { Square::new(File::C, sq.rank()) } else { qs_rook })
                        }
                    }
                }

                moves
            },
        }
    }

    fn ep_square(&self, color: Color) -> Bitboard {
        if let Some(f) = self.en_passant {
            Square::new(f, [Rank::_3, Rank::_6][!color as usize]).into()
        } else {
            Bitboard::default()
        }
    }

    pub fn is_check(&self) -> bool {
        self.is_side_check(self.side_to_move)
    }

    pub fn is_illegal(&self) -> bool {
        self.is_side_check(!self.side_to_move)
    }

    fn is_side_check(&self, color: Color) -> bool {
        let combined = self.combined();
        let ksq = self.king_of(color);

        !(knight::moves(ksq) & self.knights_of(!color)).is_empty()
            || !(pawn::captures(color, ksq) & self.pawns_of(!color)).is_empty()
            || !(king::moves(ksq) & self.kings()).is_empty()
            || !(bishop::moves(ksq, combined) & (self.bishops_of(!color) | self.queens_of(!color))).is_empty()
            || !(rook::moves(ksq, combined) & (self.rooks_of(!color) | self.queens_of(!color))).is_empty()
    }

    pub fn side_attack_def(&self, color: Color) -> Bitboard {
        let mut atkdef = Bitboard::default();
        for sq in self.color_combined(color) {
            let piece = self.piece_on(sq).unwrap();
            atkdef |= self.piece_targets(false, color, piece, sq);
        }
        atkdef
    }

    #[doc(hidden)]
    pub fn _check_legality(&self) {
        assert_eq!(self.pieces.into_iter().fold(Bitboard::default(), |a, p| {
            if !(a & p).is_empty() {
                panic!("piece table overlap");
            }

            a ^ p
        }), self.combined(), "piece tb cumul OR != color tb cumul OR");

        for sq in self.combined() {
            if let None = self.piece_on(sq) {
                panic!("{self:?}\n{sq}");
            }
        }
    }
}

fn mailbox_element(color: Color, piece: Piece) -> u8 {
    ((color as u8 + 1) << 3) | (piece as u8)
}

impl core::hash::Hash for Board {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state)
    }
}
