use criterion::{criterion_group, criterion_main, Criterion};
use dychess::prelude::*;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("make_move", |b| b.iter(|| {
        let board = Board::default();
        let m = Move::new(Square::E2, Square::E4, None);

        criterion::black_box(board.copy_make_move(m));
    }));
}
