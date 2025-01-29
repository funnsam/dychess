use dychess::prelude::*;

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

fn main() {
    perft::<true>(&Board::from_fen(false, "\
    rnbqkbnr/p1pppppp/Bp6/8/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 1 2\
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
