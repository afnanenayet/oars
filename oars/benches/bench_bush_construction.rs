use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use oars::constructors::Bush;
use oars::oa::OAConstructor;

#[cfg(feature = "parallel")]
use oars::oa::ParOAConstructor;

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
        prime_base: 11,
        dimensions: 5,
        strength: 5,
    };
    c.bench_function("Bush (base 11, dim 5, strength 5)", move |b| {
        b.iter(|| bush.gen().unwrap())
    });
}

fn bench_bush_par_small(c: &mut Criterion) {
    let bush = Bush {
        prime_base: 3,
        dimensions: 3,
        strength: 3,
    };
    c.bench_function("Bush (parallel) (base 3, dim 3, strength 3)", move |b| {
        b.iter(|| bush.gen_par().unwrap())
    });
}

fn bench_bush_par_large(c: &mut Criterion) {
    let bush = Bush {
        prime_base: 11,
        dimensions: 5,
        strength: 5,
    };
    c.bench_function("Bush (parallel) (base 11, dim 5, strength 5)", move |b| {
        b.iter(|| bush.gen_par().unwrap())
    });
}

criterion_group!(
    benches,
    bench_bush_small,
    bench_bush_par_small,
    bench_bush_large,
    bench_bush_par_large
);
criterion_main!(benches);
