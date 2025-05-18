use dychess::prelude::*;

static EPD: &str = include_str!("standard.epd");

#[test]
fn test_perft() {
    // perft::<true>(&Board::from_epd(false, "\
    // r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q2/PPPBBPpP/R3K2R b Kkq - 1 2\
    // ").unwrap(), 1);
    // panic!();

    for line in EPD.lines() {
        let (board, test) = line.split_once(" ;D").unwrap();
        let board = Board::from_epd(false, board).expect(board);

        for i in test.split(" ;D") {
            let (depth, expected) = i.split_once(' ').unwrap();
            let depth = depth.parse().unwrap();
            let expected = expected.parse().unwrap();

            assert_eq!(perft::<true>(&board, depth), expected, "{board} ; D{depth}");
        }
    }
}

fn perft<const ROOT: bool>(board: &Board, depth: usize) -> u64 {
    // board._check_legality();
    if depth == 0 { return 1 };

    let mut iter = board.pseudo_legal_moves();

    let mut chunk = Chunk::new_const();
    let mut total = 0;
    while iter.next_chunk(&mut chunk) {
        for m in &chunk {
            let mut this = *board;
            this.make_move(*m);

            if this.is_illegal() { continue };

            let this_node = perft::<false>(&this, depth - 1);
            total += this_node;

            if ROOT {
                println!("  {m}: {this_node}");
            }
        }
    }

    if ROOT {
        println!("{depth}: {total} nodes\n");
    }

    total
}
