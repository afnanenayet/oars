/// Unit tests for the Bose array constructor

use oars::constructors::Bose;
use oars::oa::OAConstructor;

#[test]
// Initialize with a non prime
fn bose_non_prime() {
    let bose = Bose { prime_base: 4, dimensions: 3};
    assert!(bose.gen().is_err());
}

#[test]
// Initialize the Bose constructor with bad `dimensions` values
fn bose_bad_dims() {
    let bose = Bose { prime_base: 5, dimensions: 1};
    assert!(bose.gen().is_err());

    let bose = Bose { prime_base: 5, dimensions: 7};
    assert!(bose.gen().is_err());

    let bose = Bose { prime_base: 13, dimensions: 20};
    assert!(bose.gen().is_err());
}
