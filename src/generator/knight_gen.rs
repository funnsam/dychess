use std::io::Write;

use crate::{bitboard::Bitboard, square::{File, Rank, Square}};

pub fn generate_moves(f: &mut impl Write) {
    write!(f, "static MOVES: [Bitboard; 64] = [").unwrap();
    for rank in Rank::ALL {
        for file in File::ALL {
            let mut bb = Bitboard::default();

            if let (Some(r), Some(f)) = (rank.up(2), file.left(1)) {
                bb |= Square::new(f, r).into();
            }
            if let (Some(r), Some(f)) = (rank.up(2), file.right(1)) {
                bb |= Square::new(f, r).into();
            }
            if let (Some(r), Some(f)) = (rank.down(2), file.left(1)) {
                bb |= Square::new(f, r).into();
            }
            if let (Some(r), Some(f)) = (rank.down(2), file.right(1)) {
                bb |= Square::new(f, r).into();
            }
            if let (Some(r), Some(f)) = (rank.up(1), file.left(2)) {
                bb |= Square::new(f, r).into();
            }
            if let (Some(r), Some(f)) = (rank.up(1), file.right(2)) {
                bb |= Square::new(f, r).into();
            }
            if let (Some(r), Some(f)) = (rank.down(1), file.left(2)) {
                bb |= Square::new(f, r).into();
            }
            if let (Some(r), Some(f)) = (rank.down(1), file.right(2)) {
                bb |= Square::new(f, r).into();
            }

            write!(f, "Bitboard({}),", bb.0).unwrap();
        }
    }
    write!(f, "];").unwrap();
}
