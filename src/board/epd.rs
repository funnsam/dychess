use core::{str::Chars, fmt};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpdError {
    UnexpectedChar(char),
    UnexpectedEnd,
    TooMuchPieces { rank: Rank },
    TooLittleRanks { last_rank: Rank },
}

impl fmt::Display for EpdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(ch) => write!(f, "unexpected character `{ch}`"),
            Self::UnexpectedEnd => write!(f, "unexpected end of EPD string"),
            Self::TooMuchPieces { rank } => write!(f, "too much pieces in rank {rank}"),
            Self::TooLittleRanks { last_rank } => write!(f, "too little ranks, last is rank {last_rank}"),
        }
    }
}

impl core::error::Error for EpdError {}

impl Board {
    /// Parse a EPD string into a board.
    ///
    /// # Note
    /// This function may not return `Err` on all invalid EPD string.
    ///
    /// # Example
    /// ```
    /// use dychess::prelude::*;
    ///
    /// let initial = Board::from_epd(
    ///     false,
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    /// ).expect("valid position");
    /// ```
    pub fn from_epd(chess960: bool, epd: &str) -> Result<Self, EpdError> {
        assert!(!chess960, "chess960 is not supported yet");

        let mut board = Self::empty();
        let mut epd = epd.chars();

        board.chess960 = chess960;
        board.parse_epd_header(&mut epd)?;

        loop {
            match epd.next() {
                // TODO: chess960 castling
                Some('K') => board.allow_king_side_castle(Color::White),
                Some('Q') => board.allow_queen_side_castle(Color::White),
                Some('k') => board.allow_king_side_castle(Color::Black),
                Some('q') => board.allow_queen_side_castle(Color::Black),
                Some(' ') => break,
                Some('-') => match epd.next() {
                    Some(' ') => break,
                    Some(ch) => return Err(EpdError::UnexpectedChar(ch)),
                    None => return Err(EpdError::UnexpectedEnd),
                },
                Some(ch) => return Err(EpdError::UnexpectedChar(ch)),
                None => return Err(EpdError::UnexpectedEnd),
            }
        }

        board.parse_epd_footer(&mut epd)?;
        Ok(board)
    }

    fn parse_epd_header(&mut self, epd: &mut Chars<'_>) -> Result<(), EpdError> {
        for rank in Rank::ALL.into_iter().rev() {
            let mut file = 0;

            loop {
                let ch = epd.next();
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
                    Some(' ') => return Err(EpdError::TooLittleRanks { last_rank: rank }),
                    Some('/') => break,
                    Some(ch) => return Err(EpdError::UnexpectedChar(ch)),
                    None => return Err(EpdError::UnexpectedEnd),
                }
                file += 1;

                if file > 8 {
                    return Err(EpdError::TooMuchPieces { rank });
                }
            }
        }

        self.side_to_move = match epd.next() {
            Some('w') => { self.hash ^= zobrist::SIDE_TO_MOVE; Color::White },
            Some('b') => Color::Black,
            Some(ch) => return Err(EpdError::UnexpectedChar(ch)),
            None => return Err(EpdError::UnexpectedEnd),
        };

        match epd.next() {
            Some(' ') => Ok(()),
            Some(ch) => Err(EpdError::UnexpectedChar(ch)),
            None => Err(EpdError::UnexpectedEnd),
        }
    }

    fn parse_epd_footer(&mut self, epd: &mut Chars<'_>) -> Result<(), EpdError> {
        self.en_passant = match epd.next() {
            Some('a') => Some(File::A),
            Some('b') => Some(File::B),
            Some('c') => Some(File::C),
            Some('d') => Some(File::D),
            Some('e') => Some(File::E),
            Some('f') => Some(File::F),
            Some('g') => Some(File::G),
            Some('h') => Some(File::H),
            Some('-') => None,
            Some(ch) => return Err(EpdError::UnexpectedChar(ch)),
            None => return Err(EpdError::UnexpectedEnd),
        };
        Ok(())
    }
}

/// Format `self` into an EPD string.
///
/// # Example
/// ```
/// use dychess::prelude::*;
///
/// let initial = Board::default();
/// assert_eq!(initial.to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -");
/// ```
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in Rank::ALL.into_iter().rev() {
            let mut empty = 0;

            for file in File::ALL {
                if let Some((piece, color)) = self.piece_and_color_on(Square::new(file, rank)) {
                    if empty != 0 { write!(f, "{empty}")? };
                    write!(f, "{}", piece.to_char(color))?;
                    empty = 0;
                } else {
                    empty += 1;
                }
            }

            if empty != 0 { write!(f, "{empty}")? };
            if rank != Rank::_1 { write!(f, "/")? };
        }

        write!(f, " {} ", self.side_to_move())?;

        let mut i = 0;
        if self.castle_rights[0].king_side()  { i += 1; write!(f, "K")? };
        if self.castle_rights[0].queen_side() { i += 1; write!(f, "Q")? };
        if self.castle_rights[1].king_side()  { i += 1; write!(f, "k")? };
        if self.castle_rights[1].queen_side() { i += 1; write!(f, "q")? };
        if i == 0 { write!(f, "-")? };

        if let Some(file) = self.en_passant {
            write!(f, " {file}{}", pawn::ep_target_rank(self.side_to_move()))
        } else {
            write!(f, " -")
        }
    }
}
