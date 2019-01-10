/// Unit tests for the Bose array constructor
use oars::constructors::Bose;
use oars::oa::{normalize, verify_oa, OAConstructor};

#[test]
fn test_bose_init_verify() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    assert!(verify_oa(&oa));

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    assert!(verify_oa(&oa));

    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    let oa = bose.gen().unwrap();
    assert!(verify_oa(&oa));
}

#[test]
fn test_bose_normalize() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    normalize(&oa, 0.0, true);
    normalize(&oa, 1.0, true);

    let bose = Bose {
        prime_base: 3,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    normalize(&oa, 0.0, true);
    normalize(&oa, 1.0, true);
}
