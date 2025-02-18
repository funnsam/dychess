#[allow(unused)]
mod bitboard;
#[allow(unused)]
mod castle_rights;
#[allow(unused)]
mod chess_move;
#[allow(unused)]
mod color;
#[allow(unused)]
mod piece;
#[allow(unused)]
mod square;

mod generator;

fn main() {
    generator::main();
}

mod bb_data {
    use crate::bitboard::Bitboard;

    pub const EDGE: Bitboard = Bitboard(0);
    pub const LEFTS: [Bitboard; 8] = [Bitboard(0); 8];
    pub const RIGHTS: [Bitboard; 8] = [Bitboard(0); 8];
}
