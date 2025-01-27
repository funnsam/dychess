use std::io::Write;

use crate::{bitboard::Bitboard, square::{File, Rank, Square}};

pub fn generate_data(f: &mut impl Write) -> Bitboard {
    generate_files(f);
    generate_ranks(f);
    generate_edge(f)
}

pub fn generate_edge(f: &mut impl Write) -> Bitboard {
    let mut edge = Bitboard::default();

    for rank in Rank::ALL {
        edge |= Square::new(File::A, rank).into();
        edge |= Square::new(File::H, rank).into();
    }
    for file in File::ALL {
        edge |= Square::new(file, Rank::_1).into();
        edge |= Square::new(file, Rank::_8).into();
    }

    write!(f, "pub const EDGE: Bitboard = Bitboard({});", edge.0).unwrap();
    edge
}

pub fn generate_files(f: &mut impl Write) {
    let mut a_file = Bitboard::default();

    for rank in Rank::ALL {
        a_file |= Square::new(File::A, rank).into();
    }

    write!(f, "pub const FILES: [Bitboard; 8] = [").unwrap();
    for _ in 0..8 {
        write!(f, "Bitboard({}),", a_file.0).unwrap();
        a_file.0 <<= 1;
    }
    write!(f, "];").unwrap();
}

pub fn generate_ranks(f: &mut impl Write) {
    let mut rank_1 = Bitboard::default();

    for file in File::ALL {
        rank_1 |= Square::new(file, Rank::_1).into();
    }

    write!(f, "pub const RANKS: [Bitboard; 8] = [").unwrap();
    for _ in 0..8 {
        write!(f, "Bitboard({}),", rank_1.0).unwrap();
        rank_1.0 <<= 8;
    }
    write!(f, "];").unwrap();
}
