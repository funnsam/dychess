use arrayvec::ArrayVec;

use super::{Bitboard, BitboardIter, Board, Move, Piece, pawn};

pub type Chunk = ArrayVec<Move, 27>;

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
    /// let mut moves = board.pseudo_legal_moves();
    ///
    /// let mut chunk = Chunk::new_const();
    /// let mut count = 0;
    /// while moves.next_chunk(&mut chunk) {
    ///     for m in &chunk {
    ///         let mut this = board;
    ///         this.make_move(*m);
    ///
    ///         count += !this.is_illegal() as u64;
    ///     }
    /// }
    ///
    /// assert_eq!(count, 20);
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

impl<'a, const CAPTURES: bool> MoveGen<'a, CAPTURES> {
    pub fn next_chunk(&mut self, chunk: &mut Chunk) -> bool {
        let target_mask = if CAPTURES { self.board.combined() } else { !Bitboard::default() };

        let square = if let Some(square) = self.pieces.next() { square } else { return false };
        let piece = self.board.piece_on(square).unwrap();

        let piece_targets = self.board.piece_targets::<false>(self.board.side_to_move(), piece, square)
            & target_mask;

        chunk.clear();

        for to in piece_targets {
            let promotes = piece == Piece::Pawn && pawn::PROMOTION_SQUARES.square_active(to);

            if promotes {
                for promote in Piece::PROMOTE_TO {
                    // SAFETY: piece targets doesnt contain from == to
                    unsafe {
                        chunk.push(Move::new_unchecked(square, to, Some(promote)));
                    }
                }
            } else {
                // SAFETY: piece targets doesnt contain from == to
                unsafe {
                    chunk.push(Move::new_unchecked(square, to, None));
                }
            }
        }

        true
    }
}
