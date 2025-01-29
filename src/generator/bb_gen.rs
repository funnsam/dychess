use std::io::Write;

use crate::{bitboard::Bitboard, square::{File, Rank, Square}};

pub fn generate_data(f: &mut impl Write) -> ([Bitboard; 8], [Bitboard; 8]) {
    generate_edge(f);
    let files = generate_files(f);
    let ranks = generate_ranks(f);
    generate_lefts(f, files);
    generate_rights(f, files);
    (files, ranks)
}

pub fn generate_edge(f: &mut impl Write) {
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
}

pub fn generate_files(f: &mut impl Write) -> [Bitboard; 8] {
    let mut a_file = Bitboard::default();

    for rank in Rank::ALL {
        a_file |= Square::new(File::A, rank).into();
    }

    let mut res = [Bitboard::default(); 8];
    write!(f, "pub const FILES: [Bitboard; 8] = [").unwrap();
    for i in 0..8 {
        write!(f, "Bitboard({}),", a_file.0).unwrap();
        res[i] = a_file;
        a_file.0 <<= 1;
    }
    write!(f, "];").unwrap();
    res
}

pub fn generate_ranks(f: &mut impl Write) -> [Bitboard; 8] {
    let mut rank_1 = Bitboard::default();

    for file in File::ALL {
        rank_1 |= Square::new(file, Rank::_1).into();
    }

    let mut res = [Bitboard::default(); 8];
    write!(f, "pub const RANKS: [Bitboard; 8] = [").unwrap();
    for i in 0..8 {
        write!(f, "Bitboard({}),", rank_1.0).unwrap();
        res[i] = rank_1;
        rank_1.0 <<= 8;
    }
    write!(f, "];").unwrap();
    res
}

pub fn generate_lefts(f: &mut impl Write, ranks: [Bitboard; 8]) {
    write!(f, "pub const LEFTS: [Bitboard; 8] = [").unwrap();

    let mut acc = Bitboard::default();
    for r in ranks {
        write!(f, "Bitboard({}),", acc.0).unwrap();
        acc |= r;
    }

    write!(f, "];").unwrap();
}

pub fn generate_rights(f: &mut impl Write, ranks: [Bitboard; 8]) {
    write!(f, "pub const RIGHTS: [Bitboard; 8] = [").unwrap();

    let mut acc = !Bitboard::default();
    for r in ranks {
        acc ^= r;
        write!(f, "Bitboard({}),", acc.0).unwrap();
    }

    write!(f, "];").unwrap();
}
