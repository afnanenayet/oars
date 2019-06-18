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
    assert!(verify(&oa));

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    assert!(verify(&oa));

    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    let oa = bose.gen().unwrap();
    assert!(verify(&oa));
}

#[test]
#[cfg(feature = "parallel")]
fn test_bose_par_init_verify() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen_par().unwrap();
    assert!(verify(&oa));

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen_par().unwrap();
    assert!(verify(&oa));

    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    let oa = bose.gen_par().unwrap();
    assert!(verify(&oa));
}

#[test]
fn test_bose_normalize() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    normalize::<u32, f64>(&oa, 0.0, true);
    normalize(&oa, 1.0, true);

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    normalize(&oa, 0.0, true);
    normalize(&oa, 1.0, true);
}
