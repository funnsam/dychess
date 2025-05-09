use super::{Bitboard, BitboardIter, Board, Move, Piece, Square, pawn};

/// A staged pseudo-legal move generator.
pub struct MoveGen<'a> {
    board: &'a Board,

    target_mask: Bitboard,
    cur_piece_targets: Bitboard,
    cur_piece_sq: Square,
    cur_promote_to: u8,

    pieces: BitboardIter,
}

impl Board {
    /// Generate pseudo-legal moves that can be iterated with a list of moves that are prioritized
    /// over other moves.
    #[inline]
    #[must_use]
    pub fn pseudo_legal_moves<'a>(&'a self) -> MoveGen<'a> {
        MoveGen {
            board: self,

            target_mask: !Bitboard::default(),
            cur_piece_targets: Bitboard::default(),
            cur_piece_sq: Square::default(),
            cur_promote_to: 0,

            pieces: self.our_pieces().into_iter(),
        }
    }

    /// Generate pseudo-legal captures that can be iterated with a list of moves that are
    /// prioritized over other moves.
    #[inline]
    #[must_use]
    pub fn pseudo_legal_captures<'a>(&'a self) -> MoveGen<'a> {
        MoveGen {
            board: self,

            target_mask: self.combined(),
            cur_piece_targets: Bitboard::default(),
            cur_piece_sq: Square::default(),
            cur_promote_to: 0,

            pieces: self.our_pieces().into_iter(),
        }
    }
}

impl MoveGen<'_> {
    #[inline]
    fn set_target(&mut self) -> Option<Square> {
        while self.cur_piece_targets.is_empty() {
            let square = self.pieces.next()?;
            let piece = self.board.piece_on(square).unwrap();

            let piece_targets = self.board.piece_targets::<false>(self.board.side_to_move(), piece, square)
                & self.target_mask;

            self.cur_piece_targets = piece_targets;
            self.cur_piece_sq = square;
        }

        // SAFETY: `self.cur_piece_targets` is checked for 0 in a loop before
        Some(unsafe { self.cur_piece_targets.first_square().unwrap_unchecked() })
    }
}

impl Iterator for MoveGen<'_> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        let to_sq = self.set_target()?;

        if self.cur_promote_to == 0 {
            let piece = self.board.piece_on(self.cur_piece_sq).unwrap();
            self.cur_promote_to = (
                piece == Piece::Pawn
                && !(self.cur_piece_targets & pawn::PROMOTION_SQUARES).is_empty()
            ) as u8;
        }

        let mov = if self.cur_promote_to == 0 {
            self.cur_piece_targets ^= to_sq.into();
            Move::new(self.cur_piece_sq, to_sq, None)
        } else {
            if self.cur_promote_to >= Piece::Queen as u8 {
                self.cur_piece_targets ^= to_sq.into();
                self.cur_promote_to = 0;
            } else {
                self.cur_promote_to += 1;
            }

            Move::new(self.cur_piece_sq, to_sq, Some(Piece::from_index(self.cur_promote_to)))
        };

        Some(mov)
    }
}

impl core::iter::FusedIterator for MoveGen<'_> {}
