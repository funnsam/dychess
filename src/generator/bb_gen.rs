use std::io::Write;

use crate::{bitboard::Bitboard, square::{File, Rank, Square}};

pub fn generate_data(f: &mut impl Write) {
    generate_edge(f);
    generate_lefts(f);
    generate_rights(f);
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

pub fn generate_lefts(f: &mut impl Write) {
    write!(f, "pub const LEFTS: [Bitboard; 8] = [").unwrap();

    let mut acc = Bitboard::default();
    for r in Rank::ALL {
        write!(f, "Bitboard({}),", acc.0).unwrap();
        acc |= r.into();
    }

    write!(f, "];").unwrap();
}

pub fn generate_rights(f: &mut impl Write) {
    write!(f, "pub const RIGHTS: [Bitboard; 8] = [").unwrap();

    let mut acc = !Bitboard::default();
    for r in Rank::ALL {
        acc ^= r.into();
        write!(f, "Bitboard({}),", acc.0).unwrap();
    }

    write!(f, "];").unwrap();
}
