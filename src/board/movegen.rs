use arrayvec::ArrayVec;

use super::{Bitboard, BitboardIter, Board, Move, Piece, pawn};

/// A staged pseudo-legal move generator.
#[derive(Clone, Copy)]
pub struct MoveGen<'a, const CAPTURES: bool> {
    board: &'a Board,
    pieces: BitboardIter,
}

impl Board {
    /// Generate pseudo-legal moves that can be iterated with a list of moves that are prioritized
    /// over other moves.
    ///
    /// # Example
    /// ```
    /// # use dychess::prelude::*;
    /// #
    /// let board = Board::default();
    ///
    /// assert_eq!(
    ///     board.pseudo_legal_moves().flatten().count(),
    ///     20,
    /// );
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn pseudo_legal_moves<'a>(&'a self) -> MoveGen<'a, false> {
        MoveGen {
            board: self,
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
            pieces: self.our_pieces().into_iter(),
        }
    }
}

impl<const CAPTURES: bool> Iterator for MoveGen<'_, CAPTURES> {
    type Item = ArrayVec<Move, 27>;

    fn next(&mut self) -> Option<ArrayVec<Move, 27>> {
        let target_mask = if CAPTURES { self.board.combined() } else { !Bitboard::default() };

        let square = self.pieces.next()?;
        let piece = self.board.piece_on(square).unwrap();

        let piece_targets = self.board.piece_targets::<false>(self.board.side_to_move(), piece, square)
            & target_mask;

        let mut moves = ArrayVec::new_const();

        for to in piece_targets {
            let promotes = piece == Piece::Pawn && pawn::PROMOTION_SQUARES.square_active(to);

            if promotes {
                for promote in Piece::PROMOTE_TO {
                    // SAFETY: piece targets doesnt contain from == to
                    unsafe {
                        moves.push(Move::new_unchecked(square, to, Some(promote)));
                    }
                }
            } else {
                // SAFETY: piece targets doesnt contain from == to
                unsafe {
                    moves.push(Move::new_unchecked(square, to, None));
                }
            }
        }

        Some(moves)
    }
}

impl<const CAPTURES: bool> core::iter::FusedIterator for MoveGen<'_, CAPTURES> {}
