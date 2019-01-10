/// Unit tests for the Bose array constructor
use oars::constructors::Bush;
use oars::oa::{normalize, verify_oa, OAConstructor};

#[test]
fn test_bush_init_verify() {
    let bush = Bush {
        prime_base: 2,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    println!("oa\n{:?}", oa.points);
    assert!(verify_oa(&oa));

    let bush = Bush {
        prime_base: 3,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    assert!(verify_oa(&oa));

    let bush = Bush {
        prime_base: 3,
        dimensions: 3,
        strength: 3,
    };
    let oa = bush.gen().unwrap();
    assert!(verify_oa(&oa));

    let bush = Bush {
        prime_base: 5,
        dimensions: 4,
        strength: 4,
    };
    let oa = bush.gen().unwrap();
    assert!(verify_oa(&oa));
}

#[test]
fn test_bush_normalize() {
    let bush = Bush {
        prime_base: 2,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    normalize(&oa, 0.0, true);
    normalize(&oa, 1.0, true);

    let bush = Bush {
        prime_base: 3,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    normalize(&oa, 0.0, true);
    normalize(&oa, 1.0, true);
}
