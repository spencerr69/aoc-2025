extern crate aoc_2025;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("one", |b| b.iter(|| aoc_2025::one::run()));
    c.bench_function("two", |b| b.iter(|| aoc_2025::two::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
