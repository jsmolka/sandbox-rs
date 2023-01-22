use bit_ops::BitOps;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bits_ext(value: u32) {
    black_box(value.bits(0..32));
    black_box(value.bits(1..31));
    black_box(value.bits(7..16));
    black_box(value.bits(9..10));
}

fn bits_naive(value: u32) {
    black_box(value);
    black_box((value >> 1) & 0x3FFF_FFFF);
    black_box((value >> 7) & 0x0000_01FF);
    black_box((value >> 9) & 0x0000_0001);
}

fn benchmark(c: &mut Criterion) {
    let value = black_box(0);
    c.bench_function("bits_ext", |b| b.iter(|| bits_ext(value)));
    c.bench_function("bits_naive", |b| b.iter(|| bits_naive(value)));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
