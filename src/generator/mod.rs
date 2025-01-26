use std::{env, fs::File, path::Path};

mod bb_gen;
mod pawn_moves_gen;
mod rays_gen;

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let rays = Path::new(&out_dir).join("rays.rs");
    rays_gen::generate_rays(&mut File::create(rays).unwrap());

    let pawn = Path::new(&out_dir).join("pawn_moves.rs");
    pawn_moves_gen::generate_moves(&mut File::create(pawn).unwrap());

    let bitboard = Path::new(&out_dir).join("bitboard.rs");
    bb_gen::generate_data(&mut File::create(bitboard).unwrap());
}
