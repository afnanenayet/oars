/// Unit tests for the Bose array constructor
use oars::constructors::Bose;
use oars::oa::{verify_oa, OAConstructor};

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
}
