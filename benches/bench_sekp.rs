use criterion::{criterion_group, criterion_main, Criterion};
use programming_bitcoin::secp256k1::*;

fn big_mult() {
    let g = SECP256K1_G.clone();
    let _ = g * SECP256K1_N.clone();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("secp256k1_mul_by_order", |b| b.iter(|| big_mult()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
