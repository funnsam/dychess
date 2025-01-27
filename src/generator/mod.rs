use std::{env, fs::File, path::Path};

mod bb_gen;
mod king_gen;
mod knight_gen;
mod magic_gen;
mod pawn_gen;
mod rays_gen;

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let bitboard = Path::new(&out_dir).join("bitboard.rs");
    let (files, ranks) = bb_gen::generate_data(&mut File::create(bitboard).unwrap());

    let rays = Path::new(&out_dir).join("rays.rs");
    let rays = rays_gen::generate_rays(&mut File::create(rays).unwrap());

    let pawn = Path::new(&out_dir).join("pawn.rs");
    pawn_gen::generate_moves(&mut File::create(pawn).unwrap());

    let knight = Path::new(&out_dir).join("knight.rs");
    knight_gen::generate_moves(&mut File::create(knight).unwrap());

    let king = Path::new(&out_dir).join("king.rs");
    king_gen::generate_tables(&mut File::create(king).unwrap());

    let magic = Path::new(&out_dir).join("magic.rs");
    magic_gen::generate_tables(&mut File::create(magic).unwrap(), files, ranks, rays);
}
