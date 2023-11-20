use concur::assignments;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn sum_of_digits_rec(c: &mut Criterion) -> &mut Criterion {
    c.bench_function("assignment", |b| {
        b.iter(|| assignments::sum_of_digits(black_box(32643565)))
    })
}
pub fn sum_of_digits_optimized(c: &mut Criterion) -> &mut Criterion {
    c.bench_function("assignment", |b| {
        b.iter(|| assignments::sum_of_digits_optimized(black_box(32643565)))
    })
}

criterion_group!(benches, sum_of_digits_rec, sum_of_digits_optimized);
criterion_main!(benches);
