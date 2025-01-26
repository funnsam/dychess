use crate::prelude::*;

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
}

impl Board {
    pub fn make_move(&mut self, mov: Move) {
        let piece = self.erase_piece(self.side_to_move, mov.from())
            .expect("tried to make invalid move: piece does not exist on move `from` square");
        _ = self.erase_piece(!self.side_to_move, mov.to());

        let to_bb = Bitboard::from(mov.to());
        self.pieces[piece as usize] |= to_bb;
        self.colors[self.side_to_move as usize] |= to_bb;
        self.mailbox[mov.to().to_usize()] = mailbox_element(self.side_to_move, piece);
        // TODO: update hash

        match piece {
            Piece::Pawn => if let Some(ep) = self.en_passant {
                self.erase_piece(!self.side_to_move, Square::new(ep, mov.from().rank()));
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
            },
            _ => {},
        }

        self.en_passant = (
            piece == Piece::Pawn
            && (mov.from().file() as u8).abs_diff(mov.to().file() as u8) == 2 // is 2 sq push
        ).then_some(mov.from().file());
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

    /// Get the piece on a given square.
    #[inline(always)]
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        let element = self.mailbox[square.to_usize()];
        (element != 0).then_some(Piece::ALL[element as usize & 7])
    }

    /// Get the color of the piece on a given square.
    #[inline(always)]
    pub fn color_on(&self, square: Square) -> Option<Color> {
        let element = self.mailbox[square.to_usize()];
        (element != 0).then_some(Color::ALL[(element as usize) >> 3])
    }

    /// All pieces combined in a bitboard.
    #[inline(always)]
    pub fn combined(&self) -> Bitboard {
        self.colors[0] | self.colors[1]
    }

    /// All pieces that belongs to the specified color.
    #[inline(always)]
    pub fn color_combined(&self, color: Color) -> Bitboard { self.colors[color as usize] }

    /// All pieces that belongs to white.
    #[inline(always)]
    pub fn white_pieces(&self) -> Bitboard { self.color_combined(Color::White) }

    /// All pieces that belongs to black.
    #[inline(always)]
    pub fn black_pieces(&self) -> Bitboard { self.color_combined(Color::Black) }

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
