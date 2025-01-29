use std::io::Write;

use crate::{bitboard::Bitboard, color::Color, square::{File, Rank, Square}};

pub fn generate_tables(f: &mut impl Write) {
    generate_moves(f);
    generate_castle_paths(f);
    generate_castle_moves(f);
}

fn generate_moves(f: &mut impl Write) {
    write!(f, "static MOVES: [Bitboard; 64] = [").unwrap();
    for rank in Rank::ALL {
        for file in File::ALL {
            let mut bb = Bitboard::default();

            for r in &Rank::ALL[(rank as usize).saturating_sub(1)..(rank as usize + 2).min(8)] {
                for f in &File::ALL[(file as usize).saturating_sub(1)..(file as usize + 2).min(8)] {
                    if *r != rank || *f != file {
                        bb |= Square::new(*f, *r).into();
                    }
                }
            }

            write!(f, "Bitboard({}),", bb.0).unwrap();
        }
    }
    write!(f, "];").unwrap();
}

fn generate_castle_paths(f: &mut impl Write) {
    write!(f, "static CASTLE_PATH: [[[Bitboard; 2]; 8]; 2] = [").unwrap();
    for color in Color::ALL {
        write!(f, "[").unwrap();
        for king_file in File::ALL {
            write!(f, "[").unwrap();
            for castle_side in [File::G, File::C] {
                let mut bb = Bitboard::default();
                if king_file > castle_side {
                    // castle to left
                    for f in File::ALL.into_iter().skip(2) {
                        bb |= Square::new(f, color.back_rank()).into();
                        if f == king_file { break };
                    }
                } else {
                    // castle to right
                    for f in File::ALL.into_iter().skip(king_file as usize) {
                        bb |= Square::new(f, color.back_rank()).into();
                        if f == castle_side { break };
                    }
                };

                write!(f, "Bitboard({}),", bb.0).unwrap();
            }
            write!(f, "],").unwrap();
        }
        write!(f, "],").unwrap();
    }
    write!(f, "];").unwrap();

    write!(f, "static CASTLE_CLEARANCE: [[[Bitboard; 2]; 8]; 2] = [").unwrap();
    for color in Color::ALL {
        write!(f, "[").unwrap();
        for king_file in File::ALL {
            write!(f, "[").unwrap();
            for castle_side in [File::G, File::C] {
                let mut bb = Bitboard::default();
                if king_file > castle_side {
                    // castle to left
                    for f in File::ALL.into_iter().skip(1) {
                        if f == king_file { break };
                        bb |= Square::new(f, color.back_rank()).into();
                    }
                } else {
                    // castle to right
                    for f in File::ALL.into_iter().skip(king_file as usize + 1) {
                        bb |= Square::new(f, color.back_rank()).into();
                        if f == castle_side { break };
                    }
                };

                write!(f, "Bitboard({}),", bb.0).unwrap();
            }
            write!(f, "],").unwrap();
        }
        write!(f, "],").unwrap();
    }
    write!(f, "];").unwrap();
}

fn generate_castle_moves(f: &mut impl Write) {
    write!(f, "
        /// All move to square combined ORed with king initial square for normal chess castling
        /// detection.
        pub const CASTLE_MOVE: Bitboard = Bitboard(
    ").unwrap();
    let mut bb = Bitboard::default();
    for color in Color::ALL {
        bb |= Square::new(File::C, color.back_rank()).into();
        bb |= Square::new(File::E, color.back_rank()).into();
        bb |= Square::new(File::G, color.back_rank()).into();
    }
    write!(f, "{});", bb.0).unwrap();
}
