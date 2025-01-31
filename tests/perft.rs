use dychess::prelude::*;

const INIT_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const POS3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
const POS4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
const POS5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
const POS6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

#[test]
fn test_perft() {
    // perft::<true>(&Board::from_fen(false, "\
    // r6r/Pppk1ppp/1b1B1nbN/nP1p4/B1P1P3/q4N2/Pp1P1RPP/R2Q2K1 w - - 2 3\
    // ").unwrap(), 2);
    // panic!();

    test(INIT_POS, &[20, 400, 8_902, 197_281, 4_865_609]); //, 119_060_324]);
    test(KIWIPETE, &[48, 2_039, 97_862, 4_085_603]); //, 193_690_690]);
    test(POS3, &[14, 191, 2_812, 43_238, 674_624, 11_030_083]); //, 178_633_661]);
    test(POS4, &[6, 264, 9_467, 422_333, 15_833_292]);
    test(POS5, &[44, 1_486, 62_379, 2_103_487]); //, 89_941_194]);
    test(POS6, &[46, 2_079, 89_890, 3_894_594]); //, 164_075_551]);
}

fn test(pos: &str, expected: &[u64]) {
    let board = Board::from_epd(false, pos).expect(pos);

    for (i, expected) in expected.into_iter().enumerate() {
        assert_eq!(perft::<true>(&board, i + 1), *expected, "{pos} ; D{}", i + 1);
    }
}

fn perft<const ROOT: bool>(board: &Board, depth: usize) -> u64 {
    // board._check_legality();
    if depth == 0 { return 1 };

    let mut total = 0;
    for m in board.pseudo_legal_moves(&[]) {
        let this = board.copy_make_move(m);
        if this.is_illegal() { continue };

        let this_node = perft::<false>(&this, depth - 1);
        total += this_node;

        // if ROOT {
        //     println!("  {m}: {this_node}");
        // }
    }

    // if ROOT {
    //     println!("{depth}: {total} nodes\n");
    // }

    total
}
