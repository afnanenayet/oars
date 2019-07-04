/// Unit tests for the Bose array constructor
use oars::constructors::Bose;
use oars::oa::{normalize, verify, OAConstructor};

#[cfg(feature = "parallel")]
use oars::oa::ParOAConstructor;

#[test]
fn test_bose_init_verify() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    assert!(verify(&oa).unwrap());

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    assert!(verify(&oa).unwrap());

    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    let oa = bose.gen().unwrap();
    assert!(verify(&oa).unwrap());
}

#[test]
#[cfg(feature = "parallel")]
fn test_bose_par_init_verify() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen_par().unwrap();
    assert!(verify(&oa).unwrap());

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen_par().unwrap();
    assert!(verify(&oa).unwrap());

    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    let oa = bose.gen_par().unwrap();
    assert!(verify(&oa).unwrap());
}

#[test]
fn test_bose_normalize() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    assert!(normalize(&oa, 0.0, true).is_ok());
    assert!(normalize(&oa, 1.0, true).is_ok());

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    assert!(normalize(&oa, 0.0, true).is_ok());
    assert!(normalize(&oa, 1.0, true).is_ok());
}
