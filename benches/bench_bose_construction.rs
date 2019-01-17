use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use oars::constructors::Bose;
use oars::oa::OAConstructor;

fn bench_bose_small(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    c.bench_function("bose (base 3, dim 3)", move |b| b.iter(|| bose.gen()));
}

fn bench_bose_large(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 13,
        dimensions: 3,
    };
    c.bench_function("bose (base 13, dim 3)", move |b| b.iter(|| bose.gen()));
}

criterion_group!(benches, bench_bose_small, bench_bose_large);
criterion_main!(benches);
