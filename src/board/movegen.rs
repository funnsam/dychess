use super::*;

/// A staged pseudo-legal move generator.
pub struct MoveGen<'a> {
    board: &'a Board,
    priority: &'a [Move],
    priority_at: usize,

    cur_piece_targets: Bitboard,
    cur_piece_sq: Square,
    cur_promote_to: u8,

    pieces: BitboardIter,
}

impl Board {
    /// Generate pseudo-legal moves that can be iterated with a list of moves that are prioritized
    /// over other moves.
    #[inline(always)]
    pub fn pseudo_legal_moves<'a>(&'a self, priority: &'a [Move]) -> MoveGen<'a> {
        MoveGen {
            board: self,
            priority,
            priority_at: 0,

            cur_piece_targets: Bitboard::default(),
            cur_piece_sq: Square::default(),
            cur_promote_to: 0,

            pieces: self.our_pieces().into_iter(),
        }
    }
}

impl<'a> MoveGen<'a> {
    #[inline(always)]
    fn try_next(&mut self) -> Option<Result<Move, ()>> {
        if self.priority_at < self.priority.len() {
            let candidate = self.priority[self.priority_at];
            self.priority_at += 1;

            if self.priority[..self.priority_at - 1].contains(&candidate) { return Some(Err(())) };

            return if let Some((piece, color)) = self.board.piece_and_color_on(candidate.from()) {
                if color != self.board.side_to_move { return Some(Err(())) };

                let targets = self.board.piece_targets::<false>(self.board.side_to_move(), piece, candidate.from());

                if !(targets & candidate.to().into()).is_empty() {
                    Some(Ok(candidate))
                } else {
                    Some(Err(()))
                }
            } else {
                Some(Err(()))
            };
        }

        while self.cur_piece_targets.is_empty() {
            let square = self.pieces.next()?;
            let piece = self.board.piece_on(square).unwrap();

            let piece_targets = self.board.piece_targets::<false>(self.board.side_to_move(), piece, square);

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
            let promotion = Piece::ALL[self.cur_promote_to as usize];
            if promotion == Piece::Queen {
                self.cur_piece_targets ^= to_sq.into();
                self.cur_promote_to = 0;
            } else {
                self.cur_promote_to += 1;
            }

            Move::new(self.cur_piece_sq, to_sq, Some(promotion))
        };

        Some((!self.priority.contains(&mov)).then_some(mov).ok_or(()))
    }
}

impl<'a> Iterator for MoveGen<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        loop {
            if let Ok(m) = self.try_next()? {
                return Some(m);
            }
        }
    }
}

impl<'a> core::iter::FusedIterator for MoveGen<'a> {}
