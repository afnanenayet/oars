use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use oars::constructors::Bush;
use oars::oa::OAConstructor;

fn bench_bush_small(c: &mut Criterion) {
    let bush = Bush {
        prime_base: 3,
        dimensions: 3,
        strength: 3,
    };
    c.bench_function("Bush (base 3, dim 3, strength 3)", move |b| {
        b.iter(|| bush.gen().unwrap())
    });
}

fn bench_bush_large(c: &mut Criterion) {
    let bush = Bush {
        prime_base: 13,
        dimensions: 10,
        strength: 10,
    };
    c.bench_function("Bush (base 13, dim 3, strength 10)", move |b| {
        b.iter(|| bush.gen().unwrap())
    });
}

criterion_group!(benches, bench_bush_small, bench_bush_large);
criterion_main!(benches);
