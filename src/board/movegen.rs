use super::{Bitboard, BitboardIter, Board, Move, Piece, Square, pawn};

/// A staged pseudo-legal move generator.
pub struct MoveGen<'a, const CAPTURES: bool> {
    board: &'a Board,

    cur_piece_targets: Bitboard,
    cur_piece_sq: Square,
    cur_promote_to: u8,

    pieces: BitboardIter,
}

impl Board {
    /// Generate pseudo-legal moves that can be iterated with a list of moves that are prioritized
    /// over other moves.
    #[inline(always)]
    #[must_use]
    pub fn pseudo_legal_moves<'a>(&'a self) -> MoveGen<'a, false> {
        MoveGen {
            board: self,

            cur_piece_targets: Bitboard::default(),
            cur_piece_sq: Square::default(),
            cur_promote_to: 0,

            pieces: self.our_pieces().into_iter(),
        }
    }

    /// Generate pseudo-legal captures that can be iterated with a list of moves that are
    /// prioritized over other moves.
    #[inline(always)]
    #[must_use]
    pub fn pseudo_legal_captures<'a>(&'a self) -> MoveGen<'a, true> {
        MoveGen {
            board: self,

            cur_piece_targets: Bitboard::default(),
            cur_piece_sq: Square::default(),
            cur_promote_to: 0,

            pieces: self.our_pieces().into_iter(),
        }
    }
}

impl<const CAPTURES: bool> Iterator for MoveGen<'_, CAPTURES> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        let target_mask = if CAPTURES { self.board.combined() } else { !Bitboard::default() };

        while self.cur_piece_targets.is_empty() {
            let square = self.pieces.next()?;
            let piece = self.board.piece_on(square).unwrap();

            let piece_targets = self.board.piece_targets::<false>(self.board.side_to_move(), piece, square)
                & target_mask;

            self.cur_piece_targets = piece_targets;
            self.cur_piece_sq = square;
        }

        if self.cur_promote_to == 0 {
            let piece = self.board.piece_on(self.cur_piece_sq).unwrap();
            self.cur_promote_to = (
                piece == Piece::Pawn
                && !(self.cur_piece_targets & pawn::PROMOTION_SQUARES).is_empty()
            ) as u8;
        }

        // SAFETY: `self.cur_piece_targets` is checked for 0 in a loop before
        let to_sq = unsafe { self.cur_piece_targets.first_square().unwrap_unchecked() };

        let mov = if self.cur_promote_to == 0 {
            self.cur_piece_targets ^= to_sq.into();
            Move::new(self.cur_piece_sq, to_sq, None)
        } else {
            // SAFETY: the index is bounded by the if-else conds following this line
            let promotion = unsafe { *Piece::ALL.get_unchecked(self.cur_promote_to as usize) };
            if promotion == Piece::Queen {
                self.cur_piece_targets ^= to_sq.into();
                self.cur_promote_to = 0;
            } else {
                self.cur_promote_to += 1;
            }

            Move::new(self.cur_piece_sq, to_sq, Some(promotion))
        };

        Some(mov)
    }
}

impl<const CAPTURES: bool> core::iter::FusedIterator for MoveGen<'_, CAPTURES> {}
