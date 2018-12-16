use ndarray::arr2;
/// Unit tests for the Bose array constructor
use oars::constructors::Bose;
use oars::oa::OAConstructor;

#[test]
// Initialize with a non prime
fn bose_non_prime() {
    let bose = Bose {
        prime_base: 4,
        dimensions: 3,
    };
    assert!(bose.gen().is_err());
}

#[test]
// Initialize the Bose constructor with bad `dimensions` values
fn bose_bad_dims() {
    let bose = Bose {
        prime_base: 5,
        dimensions: 1,
    };
    assert!(bose.gen().is_err());

    let bose = Bose {
        prime_base: 5,
        dimensions: 7,
    };
    assert!(bose.gen().is_err());

    let bose = Bose {
        prime_base: 13,
        dimensions: 20,
    };
    assert!(bose.gen().is_err());
}

#[test]
fn bose_init_2() {
    let bose = Bose {
        prime_base: 2,
        dimensions: 2,
    };
    let oa = bose.gen().unwrap();
    let ground_truth = arr2(&[[0, 0], [0, 1], [1, 0], [1, 1]]);
    assert!(oa.points == ground_truth);
}

#[test]
fn bose_init_3() {
    let bose = Bose {
        prime_base: 3,
        dimensions: 3,
    };
    let oa = bose.gen().unwrap();
    let ground_truth = arr2(&[
        [0, 0, 0],
        [0, 1, 1],
        [0, 2, 2],
        [1, 0, 0],
        [1, 1, 1],
        [1, 2, 2],
        [2, 0, 0],
        [2, 1, 1],
        [2, 2, 2],
    ]);
    assert!(oa.points == ground_truth);
}
