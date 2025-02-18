use std::io::Write;

use crate::{bitboard::Bitboard, color::Color, square::{File, Rank, Square}};

pub fn generate_moves(f: &mut impl Write) {
    generate_advances(f);
    generate_captures(f);
    write!(
        f,
        "pub const PROMOTION_SQUARES: Bitboard = Bitboard({});",
        (Bitboard::from(Rank::_1) | Bitboard::from(Rank::_8)).0,
    ).unwrap();
}

pub fn generate_advances(f: &mut impl Write) {
    write!(f, "static ADVANCES: [[Bitboard; 64]; 2] = [").unwrap();
    for color in Color::ALL {
        write!(f, "[").unwrap();
        for rank in Rank::ALL {
            let mut bb = Bitboard::from(Square::new(File::A, rank.forward_wrap(color, 1)));
            if rank.invert_if_black(color) == Rank::_2 {
                bb |= Square::new(File::A, rank.forward_wrap(color, 2)).into();
            }

            for _ in 0..8 {
                write!(f, "Bitboard({}),", bb.0).unwrap();
                bb.0 <<= 1;
            }
        }
        write!(f, "],").unwrap();
    }
    write!(f, "];").unwrap();
}

pub fn generate_captures(f: &mut impl Write) {
    write!(f, "static CAPTURES: [[Bitboard; 64]; 2] = [").unwrap();
    for color in Color::ALL {
        write!(f, "[").unwrap();
        for rank in Rank::ALL {
            for file in File::ALL {
                let fw = rank.forward_wrap(color, 1);
                let mut bb = Bitboard::default();

                if let Some(f) = file.left(1) {
                    bb |= Square::new(f, fw).into();
                }
                if let Some(f) = file.right(1) {
                    bb |= Square::new(f, fw).into();
                }

                write!(f, "Bitboard({}),", bb.0).unwrap();
            }
        }
        write!(f, "],").unwrap();
    }
    write!(f, "];").unwrap();
}
