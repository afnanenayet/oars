use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use oars::constructors::Bose;
use oars::oa::{OAConstructor, ParOAConstructor};

fn bench_bose_small(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    c.bench_function("Bose (base 3, dim 3)", move |b| {
        b.iter(|| bose.gen().unwrap())
    });
}

fn bench_bose_large(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 53,
        dimensions: 50,
    };
    c.bench_function("Bose (base 53, dims 50)", move |b| {
        b.iter(|| bose.gen().unwrap())
    });
}

fn bench_bose_small_par(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    c.bench_function("Bose (parallel) (base 3, dims 3)", move |b| {
        b.iter(|| bose.gen_par().unwrap())
    });
}

fn bench_bose_large_par(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 53,
        dimensions: 50,
    };
    c.bench_function("Bose (parallel) (base 53, dims 50)", move |b| {
        b.iter(|| bose.gen_par().unwrap())
    });
}

fn bench_bose_xlarge(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 251,
        dimensions: 250,
    };
    c.bench_function("Bose (base 251, dims 250)", move |b| {
        b.iter(|| bose.gen().unwrap())
    });
}

fn bench_bose_xlarge_par(c: &mut Criterion) {
    let bose = Bose {
        prime_base: 251,
        dimensions: 250,
    };
    c.bench_function("Bose (parallel) (base 251, dims 250)", move |b| {
        b.iter(|| bose.gen_par().unwrap())
    });
}

criterion_group!(
    benches,
    bench_bose_small,
    bench_bose_small_par,
    bench_bose_large,
    bench_bose_large_par,
    bench_bose_xlarge,
    bench_bose_xlarge_par
);
criterion_main!(benches);
