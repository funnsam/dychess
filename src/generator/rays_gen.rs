use std::io::Write;

use crate::{bitboard::Bitboard, square::{File, Rank, Square}};

pub fn generate_rays(f: &mut impl Write) -> [[Bitboard; 64]; 2] {
    let bishop = generate_bishop(f);
    let rook = generate_rook(f);
    [bishop, rook]
}

pub fn generate_bishop(f: &mut impl Write) -> [Bitboard; 64] {
    let mut rays = [Bitboard::default(); 64];
    write!(f, "static BISHOP_RAYS: [Bitboard; 64] = [").unwrap();

    for rank in Rank::ALL {
        for file in File::ALL {
            let mut bb = Bitboard::default();

            for rank2 in Rank::ALL {
                for file2 in File::ALL {
                    if (rank2 as i8 - rank as i8).abs() == (file2 as i8 - file as i8).abs()
                        && (rank != rank2 && file != file2)
                    {
                        bb |= Square::new(file2, rank2).into();
                    }
                }
            }

            write!(f, "Bitboard({}),", bb.0).unwrap();
            rays[Square::new(file, rank).to_usize()] = bb;
        }
    }

    write!(f, "];").unwrap();
    rays
}

pub fn generate_rook(f: &mut impl Write) -> [Bitboard; 64] {
    let mut rays = [Bitboard::default(); 64];
    write!(f, "static ROOK_RAYS: [Bitboard; 64] = [").unwrap();

    for rank in Rank::ALL {
        for file in File::ALL {
            let mut bb = Bitboard::default();

            for rank2 in Rank::ALL {
                if rank != rank2 {
                    bb |= Square::new(file, rank2).into();
                }
            }

            for file2 in File::ALL {
                if file != file2 {
                    bb |= Square::new(file2, rank).into();
                }
            }

            write!(f, "Bitboard({}),", bb.0).unwrap();
            rays[Square::new(file, rank).to_usize()] = bb;
        }
    }

    write!(f, "];").unwrap();
    rays
}
