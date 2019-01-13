//! The generic interface to define a strong orthogonal array (SOA) and a trait for constructing SOAs
//! This module also defines a few construction methods, as well as provide a verification method to
//! ensure that the resulting points are stratified as an SOA should be.

use ndarray::Array2;
use serde_derive::{Deserialize, Serialize};

/// The general categories of errors for `SOAConstructionError`
#[derive(Debug, Serialize, Deserialize)]
pub enum SOACErrorKind {
    /// Invalid parameters were supplied to the constructor
    InvalidParams,

    /// There was a runtime error that prevented the strong orthogonal array from being properly
    /// constructed
    RuntimeError,
}

/// An error indicating that there was some error constructing the strong orthogonal array.
#[derive(Debug)]
pub struct SOAConstructionError {
    /// The general category of the error
    error_type: SOACErrorKind,

    /// A user-friendly description of the array which can supply additional information about
    /// the error.
    desc: String,
}

/// A result type for strong orthogonal array construction
pub type SOAResult = Result<SOA, SOAConstructionError>;

/// A trait that demarcates SOA constructors
pub trait SOAConstructor {
    /// The method that generates an SOA. Any verification for the parameters must be handled by
    /// the constructor itself, and there are no generic interfaces for doing so.
    fn gen(&self) -> SOAResult;
}

/// A structure representing a strong orthogonal array, consisting of the array and associated
/// metadata.
#[derive(Debug)]
pub struct SOA {
    /// The strength of the strong orthogonal array
    pub strength: u32,

    /// The base `s` used to derive the number of levels and total number of samples
    pub base: u32,

    /// The internal array that holds the data for the strong orthogonal array
    pub points: Array2<u32>,
}

/// A nested two-dimensional vector
type Vec2D<T> = Vec<Vec<T>>;

/// Recursive utility method to determine the combinations of numbers that add up to some given
/// sum.
///
/// The sum is the target sum. The reduced number is the target after a number has already
/// been tried. `arr` is the current array of numbers that add up to the sum for the stack,
/// and `res` is a reference to an array of vectors with the results.
fn sum_perms_helper(sum: u32, reduced_num: u32, arr: Vec<u32>, res: &mut Vec2D<u32>) {
    if reduced_num == 0 {
        res.push(arr.clone());
    }

    // the previous number stored in the array
    let prev = *arr.last().unwrap_or(&1);

    for k in prev..=sum {
        let mut next_arr = arr.clone();
        next_arr.push(k);

        if k <= reduced_num {
            sum_perms_helper(sum, reduced_num - k, next_arr, res);
        }
    }
}

/// Given some desired sum, find all of the combinations of numbers that add up to the desired
/// sum. This is used to generat the strata when verifying a strong orthogonal array.
///
/// This method is a convenience wrapper for the recursive solver.
fn sum_perms(sum: u32) -> Vec2D<u32> {
    let mut res = Vec::new();
    let arr = Vec::new();
    sum_perms_helper(sum, sum, arr, &mut res);
    res
}

/// Verify whether a point set is a valid strong orthogonal array based on the metadata supplied in
/// that struct. This method returns whether the given SOA is valid, based on the metadata. It will
/// check that the SOA maintains the stratification guarantees based on the properties of the SOA.
pub fn verify_soa(soa: &SOA) -> bool {
    // TODO(afnan)
    // - Collapse the OA and test each strata
    // - Write some method that generates the unshuffled stratification guarantees
    // - Check that each strata are equally filled
    // - Write unit tests
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn test_sum_perms_ground_truth() {
        let res = sum_perms(5);
        let res_set: HashSet<Vec<u32>> = res.iter().cloned().collect();
        let ground_truth = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 2],
            vec![1, 2, 2],
            vec![1, 1, 3],
            vec![2, 3],
            vec![1, 4],
            vec![5],
        ];

        for array in ground_truth {
            assert!(res_set.contains(&array));
        }
    }

    #[test]
    fn test_sum_perms_random() {
        let mut rng = thread_rng();
        let mut targets: Vec<u32> = Vec::new();

        for _ in 0..10 {
            targets.push(rng.gen_range(1, 25));
        }

        for target in targets {
            let res: Vec2D<u32> = sum_perms(target);

            for array in res {
                assert!(array.into_iter().sum::<u32>() == target);
            }
        }
    }
}
