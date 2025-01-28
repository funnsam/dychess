use super::*;

/// A staged pseudo-legal move generator.
pub struct MoveGen<'a> {
    board: &'a Board,
    priority: &'a [Move],
    priority_at: usize,

    cur_piece_targets: Bitboard,
    cur_piece_sq: Square,

    pieces: BitboardIter,
}

impl Board {
    /// Generate pseudo-legal moves that can be iterated with a list of moves that are prioritized
    /// over other moves.
    pub fn generate_moves<'a>(&'a self, priority: &'a [Move]) -> MoveGen<'a> {
        let mut to = [Bitboard::default(); 64];

        let mut pieces = self.our_pieces().into_iter();
        let cur_piece_sq = pieces.next().expect("expected at least 1 piece avalible");

        MoveGen {
            board: self,
            priority,
            priority_at: 0,

            cur_piece_targets: self.piece_targets(cur_piece_sq),
            cur_piece_sq,

            pieces,
        }
    }

    fn piece_targets(&self, sq: Square) -> Bitboard {
        match self.piece_on(sq).unwrap() {
            Piece::Pawn => {
                // TODO: promotion, ep, blocked double
                let advances = pawn::advances(self.side_to_move(), sq);
                let captures = pawn::captures(self.side_to_move(), sq);
                advances | (captures & self.their_pieces())
            },
            Piece::Knight => knight::moves(sq),
            Piece::Bishop => bishop::moves(sq, self.combined()),
            Piece::Rook => rook::moves(sq, self.combined()),
            Piece::Queen => queen::moves(sq, self.combined()),
            Piece::King => {
                // TODO: castling
                king::moves(sq)
            },
        } & !self.our_pieces();
    }
}

impl<'a> MoveGen<'a> {
    fn try_next(&mut self) -> Option<Result<Move, ()>> {
        if self.priority_at < self.priority.len() {
            let candidate = self.priority[self.priority_at];
            self.priority_at += 1;

            // let c_idx = candidate.from().to_usize();
            // return (!(self.to[c_idx] & candidate.to().into()).is_empty())
            //     .then(|| {
            //         self.to[c_idx] &= !Bitboard::from(candidate.to());
            //         candidate
            //     })
            //     .ok_or(())
            //     .into();
        }
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
