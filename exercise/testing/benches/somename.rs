use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh(8, 9, 10)", |b| b.iter(|| sploosh(black_box(8),black_box(9),black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// V1 (Original)
//  Running benches/somename.rs (target/release/deps/somename-faeaf30f96f0071c)
// Gnuplot not found, using plotters backend
// sploosh(8, 9, 10)       time:   [788.50 ps 793.53 ps 798.88 ps]                               
// Found 6 outliers among 100 measurements (6.00%)
//   4 (4.00%) high mild
//   2 (2.00%) high severe

// V2