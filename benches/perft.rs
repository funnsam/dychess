use criterion::{criterion_group, criterion_main, Criterion};
use dychess::prelude::*;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("perft", |b| b.iter(|| {
        let board = Board::default();

        for (i, expected) in [20, 400, 8_902, 197_281, 4_865_609].into_iter().enumerate() {
            assert_eq!(perft::<true>(&board, i + 1), expected, "D{}", i + 1);
        }
    }));
}

fn perft<const ROOT: bool>(board: &Board, depth: usize) -> u64 {
    if depth == 0 { return 1 };

    let mut total = 0;
    for chunk in board.pseudo_legal_moves() {
        for m in chunk {
            let mut this = *board;
            this.make_move(m);

            if this.is_illegal() { continue };

            let this_node = perft::<false>(&this, depth - 1);
            total += this_node;
        }
    }

    total
}
