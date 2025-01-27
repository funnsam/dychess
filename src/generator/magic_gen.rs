use std::io::Write;

use crate::{bitboard::Bitboard, square::*};

#[derive(Debug)]
struct Magic {
    #[allow(unused)]
    mask: Bitboard,
    mul: u64,
    bits: u8,
}

type Table = Vec<(Bitboard, Bitboard)>;

pub fn generate_tables(f: &mut impl Write, files: [Bitboard; 8], ranks: [Bitboard; 8], masks: [[Bitboard; 64]; 2]) {
    gen(f, "BISHOP", files, ranks, masks[0], bishop_block);
    gen(f, "ROOK", files, ranks, masks[1], rook_block);
}

fn bishop_block(blockers: Bitboard, sq: Square) -> Bitboard {
    let mut res = Bitboard::default();

    for (f, r) in File::ALL[..sq.file() as usize].into_iter().rev().zip(Rank::ALL[..sq.rank() as usize].into_iter().rev()) {
        let bb = Square::new(*f, *r).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }

    for (f, r) in File::ALL[..sq.file() as usize].into_iter().rev().zip(Rank::ALL[sq.rank() as usize..].into_iter().skip(1)) {
        let bb = Square::new(*f, *r).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }

    for (f, r) in File::ALL[sq.file() as usize..].into_iter().skip(1).zip(Rank::ALL[..sq.rank() as usize].into_iter().rev()) {
        let bb = Square::new(*f, *r).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }

    for (f, r) in File::ALL[sq.file() as usize..].into_iter().skip(1).zip(Rank::ALL[sq.rank() as usize..].into_iter().skip(1)) {
        let bb = Square::new(*f, *r).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }

    res
}

fn rook_block(blockers: Bitboard, sq: Square) -> Bitboard {
    let mut res = Bitboard::default();

    for f in File::ALL[..sq.file() as usize].into_iter().rev() {
        let bb = Square::new(*f, sq.rank()).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }
    for f in File::ALL[sq.file() as usize..].into_iter().skip(1) {
        let bb = Square::new(*f, sq.rank()).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }

    for r in Rank::ALL[..sq.rank() as usize].into_iter().rev() {
        let bb = Square::new(sq.file(), *r).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }
    for r in Rank::ALL[sq.rank() as usize..].into_iter().skip(1) {
        let bb = Square::new(sq.file(), *r).into();
        res |= bb;

        if (bb & blockers).0 != 0 { break };
    }

    res
}

pub fn gen<F: Fn(Bitboard, Square) -> Bitboard>(
    f: &mut impl Write,
    name: &str,
    files: [Bitboard; 8],
    ranks: [Bitboard; 8],
    masks: [Bitboard; 64],
    block: F,
) {
    write!(f, "pub(crate) static {name}: [(Magic, &[Bitboard]); 64] = [").unwrap();
    for (mut mask, sq) in masks.into_iter().zip(Square::ALL) {
        if sq.file() != File::A { mask &= !files[0] };
        if sq.file() != File::H { mask &= !files[7] };
        if sq.rank() != Rank::_1 { mask &= !ranks[0] };
        if sq.rank() != Rank::_8 { mask &= !ranks[7] };

        let (bbs, magic) = find_magic(mask, gen_blocker_tb(mask, sq, &block));

        write!(f, "({magic:?}, &[").unwrap();
        for bb in bbs {
            write!(f, "Bitboard({}),", bb.0).unwrap();
        }
        write!(f, "]),").unwrap();
    }
    write!(f, "];").unwrap();
}

fn gen_blocker_tb<F: Fn(Bitboard, Square) -> Bitboard>(mask: Bitboard, sq: Square, res: F) -> Table {
    let mut table = Vec::with_capacity(1 << mask.popcnt());

    for i in 0..(1 << mask.popcnt()) {
        let blockers = shift_bits_into(mask, i);
        table.push((blockers, res(blockers, sq)));
    }

    table
}

fn find_magic(mask: Bitboard, table: Table) -> (Vec<Bitboard>, Magic) {
    let bits = mask.popcnt() as u8;

    'find_magic: loop {
        let trial_mul = random_u64_few_bits();
        let trial_magic = Magic {
            mask,
            mul: trial_mul,
            bits,
        };

        if mask.0.wrapping_mul(trial_mul).count_ones() < 6 {
            continue;
        }

        let mut used = vec![Bitboard::default(); 1 << bits];
        let mut max_idx = 0;

        for (blockers, movable) in &table {
            let idx = to_index(*blockers, &trial_magic);

            if used[idx] == Bitboard::default() {
                used[idx] = *movable;
                max_idx = max_idx.max(idx);
            } else if used[idx] != *movable {
                continue 'find_magic;
            }
        }

        used.drain(max_idx..);
        return (used, trial_magic);
    }
}

fn random_u64_few_bits() -> u64 {
    fastrand::u64(..) & fastrand::u64(..) & fastrand::u64(..)
}

fn to_index(bb: Bitboard, magic: &Magic) -> usize {
    (bb.0.wrapping_mul(magic.mul) >> (64 - magic.bits)) as usize
}

fn shift_bits_into(mut bb: Bitboard, mut i: u64) -> Bitboard {
    let mut result = Bitboard::default();
    for b in 0..64 {
        if bb.0 & 1 != 0 {
            result.0 |= (i & 1) << b;
            i >>= 1;
        }
        bb.0 >>= 1;
    }
    result
}
