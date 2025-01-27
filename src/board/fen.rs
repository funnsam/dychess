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
    pub fn from_fen(&self, fen: &str) -> Result<Self, FenError> {
        let mut board = Self::empty();
        let fen = fen.chars();

        let idx = board.parse_fen_header(fen)?;

        loop {
            match fen.next() {
                Some('K') => self.castle_rights[0].allow_king_side(),
                Some('Q') => self.castle_rights[0].allow_queen_side(),
                Some('k') => self.castle_rights[1].allow_king_side(),
                Some('q') => self.castle_rights[1].allow_queen_side(),
                Some('-') => break,
                Some(ch) => return Err(FenError::UnexpectedChar(ch)),
                None => return Err(FenError::UnexpectedEnd),
            }
        }
    }

    fn parse_fen_header(&mut self, fen: Chars<'_>) -> Result<(), FenError> {
        for rank in Rank::ALL.into_iter().rev() {
            let mut file = 0;

            loop {
                let ch = fen.next();
                match ch {
                    Some('1'..='8') => file += ch.unwrap().to_digit(10).unwrap() - 1,
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

                if file >= 8 {
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
}
