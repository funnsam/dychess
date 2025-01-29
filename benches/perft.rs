use dychess::prelude::*;

const INIT_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const POS3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
const POS4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POS5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const POS6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

fn main() {
    // perft::<true>(&Board::from_fen(false, "\
    // r3k2r/p1ppqpb1/1n2pnp1/1b1PN3/1p2P3/5Q1p/PPPBBPPP/RN2K2R w KQkq - 2 2\
    // ").unwrap(), 1);
    // panic!();

    // test(INIT_POS, &[20, 400, 8_902, 197_281, 4_865_609, 119_060_324]);
    test(KIWIPETE, &[48, 2039, 97862, 4085603, 193690690, 8031647685]);
}

fn test(pos: &str, expected: &[u64]) {
    let board = Board::from_fen(false, pos).expect(pos);

    for (i, expected) in expected.into_iter().enumerate() {
        assert_eq!(perft::<true>(&board, i + 1), *expected);
    }
}

fn perft<const ROOT: bool>(board: &Board, depth: usize) -> u64 {
    board._check_legality();
    if depth == 0 { return 1 };

    let mut total = 0;
    for m in board.generate_moves(&[]) {
        // println!("{:<1$}{m}", "", 10 - depth);
        let this = board.copy_make_move(m);
        if this.is_illegal() { continue };

        let this_node = perft::<false>(&this, depth - 1);
        total += this_node;

        if ROOT {
            println!("  {m}: {this_node}");
        }
    }

    if ROOT {
        println!("{depth}: {total} nodes\n");
    }

    total
}
