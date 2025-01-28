use core::{str::Chars, fmt};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenError {
    UnexpectedChar(char),
    UnexpectedEnd,
    TooMuchPieces { rank: Rank },
    TooLittleRanks { last_rank: Rank },
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(ch) => write!(f, "unexpected character `{ch}`"),
            Self::UnexpectedEnd => write!(f, "unexpected end of FEN string"),
            Self::TooMuchPieces { rank } => write!(f, "too much pieces in rank {rank}"),
            Self::TooLittleRanks { last_rank } => write!(f, "too little ranks, last is rank {last_rank}"),
        }
    }
}

impl core::error::Error for FenError {}

impl Board {
    /// Parse a FEN string into a board.
    ///
    /// # Note
    /// This function may not return `Err` on all invalid FEN string.
    ///
    /// # Example
    /// ```
    /// use dychess::prelude::*;
    ///
    /// let initial = Board::from_fen(
    ///     false,
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    /// ).expect("valid position");
    /// ```
    pub fn from_fen(chess960: bool, fen: &str) -> Result<Self, FenError> {
        let mut board = Self::empty();
        let mut fen = fen.chars();

        board.chess960 = chess960;
        board.parse_fen_header(&mut fen)?;

        loop {
            match fen.next() {
                // TODO: chess960 castling
                Some('K') => board.castle_rights[0].allow_king_side(),
                Some('Q') => board.castle_rights[0].allow_queen_side(),
                Some('k') => board.castle_rights[1].allow_king_side(),
                Some('q') => board.castle_rights[1].allow_queen_side(),
                Some(' ') => break,
                Some('-') => match fen.next() {
                    Some(' ') => break,
                    Some(ch) => return Err(FenError::UnexpectedChar(ch)),
                    None => return Err(FenError::UnexpectedEnd),
                },
                Some(ch) => return Err(FenError::UnexpectedChar(ch)),
                None => return Err(FenError::UnexpectedEnd),
            }
        }

        board.parse_fen_footer(&mut fen)?;
        Ok(board)
    }

    fn parse_fen_header(&mut self, fen: &mut Chars<'_>) -> Result<(), FenError> {
        for rank in Rank::ALL.into_iter().rev() {
            let mut file = 0;

            loop {
                let ch = fen.next();
                match ch {
                    Some('1'..='8') => file += ch.unwrap().to_digit(10).unwrap() as usize - 1,
                    Some('P') => self.place_unchecked(Color::White, Square::new(File::ALL[file], rank), Piece::Pawn),
                    Some('p') => self.place_unchecked(Color::Black, Square::new(File::ALL[file], rank), Piece::Pawn),
                    Some('N') => self.place_unchecked(Color::White, Square::new(File::ALL[file], rank), Piece::Knight),
                    Some('n') => self.place_unchecked(Color::Black, Square::new(File::ALL[file], rank), Piece::Knight),
                    Some('B') => self.place_unchecked(Color::White, Square::new(File::ALL[file], rank), Piece::Bishop),
                    Some('b') => self.place_unchecked(Color::Black, Square::new(File::ALL[file], rank), Piece::Bishop),
                    Some('R') => self.place_unchecked(Color::White, Square::new(File::ALL[file], rank), Piece::Rook),
                    Some('r') => self.place_unchecked(Color::Black, Square::new(File::ALL[file], rank), Piece::Rook),
                    Some('Q') => self.place_unchecked(Color::White, Square::new(File::ALL[file], rank), Piece::Queen),
                    Some('q') => self.place_unchecked(Color::Black, Square::new(File::ALL[file], rank), Piece::Queen),
                    Some('K') => self.place_unchecked(Color::White, Square::new(File::ALL[file], rank), Piece::King),
                    Some('k') => self.place_unchecked(Color::Black, Square::new(File::ALL[file], rank), Piece::King),
                    Some(' ') if rank == Rank::_1 => break,
                    Some(' ') => return Err(FenError::TooLittleRanks { last_rank: rank }),
                    Some('/') => break,
                    Some(ch) => return Err(FenError::UnexpectedChar(ch)),
                    None => return Err(FenError::UnexpectedEnd),
                }
                file += 1;

                if file > 8 {
                    return Err(FenError::TooMuchPieces { rank });
                }
            }
        }

        self.side_to_move = match fen.next() {
            Some('w') => Color::White,
            Some('b') => Color::Black,
            Some(ch) => return Err(FenError::UnexpectedChar(ch)),
            None => return Err(FenError::UnexpectedEnd),
        };

        match fen.next() {
            Some(' ') => Ok(()),
            Some(ch) => Err(FenError::UnexpectedChar(ch)),
            None => Err(FenError::UnexpectedEnd),
        }
    }

    fn parse_fen_footer(&mut self, fen: &mut Chars<'_>) -> Result<(), FenError> {
        self.en_passant = match fen.next() {
            Some('a') => Some(File::A),
            Some('b') => Some(File::B),
            Some('c') => Some(File::C),
            Some('d') => Some(File::D),
            Some('e') => Some(File::E),
            Some('f') => Some(File::F),
            Some('g') => Some(File::G),
            Some('h') => Some(File::H),
            Some('-') => None,
            Some(ch) => return Err(FenError::UnexpectedChar(ch)),
            None => return Err(FenError::UnexpectedEnd),
        };
        fen.next().ok_or(FenError::UnexpectedEnd)?;
        Ok(())
    }
}
