use collapse_if_slowdown::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn slow_fast_benchmark(c: &mut Criterion) {
    c.benchmark_group("slow_vs_fast");

    c.bench_function("slow", |bencher| {
        bencher.iter(|| {
            slow(black_box(0));
        })
    });

    c.bench_function("fast", |bencher| {
        bencher.iter(|| {
            fast(black_box(0));
        })
    });
}

criterion_group!(benches, slow_fast_benchmark);
criterion_main!(benches);
