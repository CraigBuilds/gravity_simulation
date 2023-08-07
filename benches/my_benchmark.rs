#![feature(anonymous_lifetime_in_impl_trait)]

use criterion::{criterion_group, criterion_main, Criterion};
use craig_iter::{IntoNextWithRestIterator, LendingIterator};

struct A;
struct B;
struct C;
struct D;

enum Enum {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl Enum {
    fn update(&mut self, _others: impl Iterator<Item = &Self>) {
        match self {
            Enum::A(_a) => {},
            Enum::B(_b) => {},
            Enum::C(_c) => {},
            Enum::D(_d) => {},
        }
    }
}

fn test_iter() {
    let mut items = vec![Enum::A(A), Enum::B(B), Enum::C(C), Enum::D(D)];
    let mut iter = items.iter_with_rest();
    while let Some((item, others)) = iter.next() {
        item.update(others);
    }
}


fn test_c_style() {
    let mut items = vec![Enum::A(A), Enum::B(B), Enum::C(C), Enum::D(D)];
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            unsafe{
                let ptr = items.as_mut_ptr();
                let a = ptr.add(i).as_mut().unwrap();
                let b = ptr.add(j).as_mut().unwrap();
                match (a, b) {
                    (Enum::A(_a), Enum::B(_b)) => {},
                    (Enum::A(_a), Enum::C(_c)) => {},
                    (Enum::A(_a), Enum::D(_d)) => {},
                    (Enum::B(_b), Enum::C(_c)) => {},
                    (Enum::B(_b), Enum::D(_d)) => {},
                    (Enum::C(_c), Enum::D(_d)) => {},
                    _ => {},
                }
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test_iter", |b| b.iter(|| test_iter()));
    c.bench_function("test_c_style", |b| b.iter(|| test_c_style()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);