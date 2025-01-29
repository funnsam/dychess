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
        let piece = self.erase_piece(self.side_to_move(), mov.from())
            .expect("tried to make invalid move: piece does not exist on move `from` square");
        let capture = self.place_piece(self.side_to_move(), mov.to(), piece);

        match piece {
            Piece::Pawn => {
                // TODO: handle ep
                // if let Some(ep) = self.en_passant {
                //     self.erase_piece(!self.side_to_move, Square::new(ep, mov.from().rank()));
                // }
            },
            Piece::Rook => {
                if (mov.from() == Square::A1 && self.side_to_move == Color::White) ||
                    (mov.from() == Square::A8 && self.side_to_move == Color::Black)
                {
                    self.castle_rights[self.side_to_move as usize].disallow_queen_side()
                } else if (mov.from() == Square::H1 && self.side_to_move == Color::White) ||
                    (mov.from() == Square::H8 && self.side_to_move == Color::Black)
                {
                    self.castle_rights[self.side_to_move as usize].disallow_king_side()
                }
            },
            Piece::King => {
                self.castle_rights[self.side_to_move as usize].disallow_castling();

                if (self.chess960 && capture == Some((Piece::Rook, self.side_to_move)))
                    || (!self.chess960 && (move_bb & king::CASTLE_MOVE) == move_bb)
                {
                    todo!("castling");
                }
            },
            _ => {},
        }

        self.en_passant = (piece == Piece::Pawn && (move_bb & pawn::double_pushes(self.side_to_move)) == move_bb)
            .then_some(mov.from().file());
        self.side_to_move = !self.side_to_move;
    }

    fn place_piece(&mut self, color: Color, square: Square, piece: Piece) -> Option<(Piece, Color)> {
        if let Some((cp, cc)) = self.piece_and_color_on(square) {
            self.erase_piece(cc, square);
            self.place_unchecked(color, square, piece);

            Some((cp, cc))
        } else {
            self.place_unchecked(color, square, piece);

            None
        }
    }

    fn place_unchecked(&mut self, color: Color, square: Square, piece: Piece) {
        let to_bb = Bitboard::from(square);
        self.pieces[piece as usize] |= to_bb;
        self.colors[color as usize] |= to_bb;
        self.mailbox[square.to_usize()] = mailbox_element(color, piece);
        // TODO: update hash
    }

    fn erase_piece(&mut self, color: Color, square: Square) -> Option<Piece> {
        let piece = self.piece_on(square);

        if let Some(piece) = piece {
            let bb = Bitboard::from(square);
            self.pieces[piece as usize] &= !bb;
            self.colors[color as usize] &= !bb;
            self.mailbox[square.to_usize()] = 0;
            // TODO: update hash
        }

        piece
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
