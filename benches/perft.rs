use dychess::prelude::*;

fn perft<const ROOT: bool>(board: &Board, depth: usize) -> u64 {
    if depth == 0 { return 1 };

    let mut total = 0;
    for m in board.generate_moves(&[]) {
        let this_node = perft::<false>(&board.copy_make_move(m), depth - 1);
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

fn main() {
    perft::<true>(&Board::from_fen(false, "\
    rnbqkbnr/1ppppppp/p7/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 2\
    ").unwrap(), 1);
    initial_pos();
}

fn initial_pos() {
    let board = Board::default();

    assert_eq!(perft::<true>(&board, 1), 20);
    assert_eq!(perft::<true>(&board, 2), 400);
    assert_eq!(perft::<true>(&board, 3), 8_902);
    assert_eq!(perft::<true>(&board, 4), 197_281);
    assert_eq!(perft::<true>(&board, 5), 4_685_609);
}
