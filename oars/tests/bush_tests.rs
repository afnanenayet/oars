/// Unit tests for the Bose array constructor
use oars::constructors::Bush;
use oars::oa::{normalize, verify, OAConstructor};

#[cfg(feature = "parallel")]
use oars::oa::ParOAConstructor;

#[test]
fn test_bush_init_verify() {
    let bush = Bush {
        prime_base: 2,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    assert!(verify(&oa).unwrap());

    let bush = Bush {
        prime_base: 3,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    assert!(verify(&oa).unwrap());

    let bush = Bush {
        prime_base: 3,
        dimensions: 3,
        strength: 3,
    };
    let oa = bush.gen().unwrap();
    assert!(verify(&oa).unwrap());

    let bush = Bush {
        prime_base: 5,
        dimensions: 4,
        strength: 4,
    };
    let oa = bush.gen().unwrap();
    assert!(verify(&oa).unwrap());
}

#[test]
#[cfg(feature = "parallel")]
fn test_bush_par_init_verify() {
    let bush = Bush {
        prime_base: 2,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen_par().unwrap();
    assert!(verify(&oa).unwrap());

    let bush = Bush {
        prime_base: 3,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen_par().unwrap();
    assert!(verify(&oa).unwrap());

    let bush = Bush {
        prime_base: 3,
        dimensions: 3,
        strength: 3,
    };
    let oa = bush.gen_par().unwrap();
    assert!(verify(&oa).unwrap());

    let bush = Bush {
        prime_base: 5,
        dimensions: 4,
        strength: 4,
    };
    let oa = bush.gen_par().unwrap();
    assert!(verify(&oa).unwrap());
}

#[test]
fn test_bush_normalize() {
    let bush = Bush {
        prime_base: 2,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    assert!(normalize(&oa, 0.0, true).is_ok());
    assert!(normalize(&oa, 1.0, true).is_ok());

    let bush = Bush {
        prime_base: 3,
        dimensions: 2,
        strength: 2,
    };
    let oa = bush.gen().unwrap();
    assert!(normalize(&oa, 0.0, true).is_ok());
    assert!(normalize(&oa, 1.0, true).is_ok());
}
