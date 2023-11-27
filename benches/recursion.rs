use concur::assignments;
use criterion::{black_box, Criterion};
pub fn sum_of_digits_rec(c: &mut Criterion) -> &mut Criterion {
    c.bench_function("sum_of_digits_rec", |b| {
        b.iter(|| assignments::sum_of_digits(black_box(32643565)))
    });
    c.bench_function("sum_of_digits_optimized", |b| {
        b.iter(|| assignments::sum_of_digits_optimized(black_box(32643565)))
    })
}
pub fn pow(c: &mut Criterion) -> &mut Criterion {
    c.bench_function("pow_rec", |b| {
        b.iter(|| assignments::pow_rec(black_box(32), black_box(3)))
    });
    c.bench_function("pow", |b| {
        b.iter(|| assignments::pow(black_box(35), black_box(21)))
    })
}

pub fn gcd(c: &mut Criterion) -> &mut Criterion {
    c.bench_function("gcd", |b| {
        b.iter(|| assignments::gcd(black_box(32), black_box(12)))
    })
}
pub fn decimal_to_binary(c: &mut Criterion) -> &mut Criterion {
    c.bench_function("decimal_to_binary", |b| {
        b.iter(|| assignments::decimal_to_binary(black_box(1222)))
    })
}