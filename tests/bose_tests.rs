/// Unit tests for the Bose array constructor
use oars::constructors::Bose;
use oars::oa::{verify_oa, OAConstructor};

#[test]
fn test_bose_init_verify() {
    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    let oa = bose.gen().unwrap();
    assert!(verify_oa(&oa));
}
