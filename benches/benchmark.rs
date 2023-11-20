use criterion::{criterion_group, criterion_main};
mod recursion;

use recursion::{decimal_to_binary, gcd, pow, sum_of_digits_rec};

criterion_group!(benches, sum_of_digits_rec, pow, gcd, decimal_to_binary);
criterion_main!(benches);
