//! Generic interfaces and definitions for strong orthogonal arrays (SOAs).
//!
//! This module defines the generic interface to define a strong orthogonal array (SOA) and a trait
//! for constructing SOAs.  This module also defines a few construction methods, as well as provide
//! a verification method to ensure that the resulting points are stratified as an SOA should be.

use crate::utils::OarsError;
use itertools::{zip, Itertools};
use ndarray::Array2;
use oars_proc_macro::Checked;
use std::collections::{HashMap, HashSet};

#[cfg(feature = "serialize")]
use serde_derive::{Deserialize, Serialize};

/// A result type for strong orthogonal array construction
pub type SOAResult = Result<SOA, OarsError>;

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
fn sum_perms_helper(sum: u32, reduced_num: u32, arr: &[u32], res: &mut Vec2D<u32>) {
    if reduced_num == 0 {
        res.push(arr.to_vec());
    }

    // the previous number stored in the array
    let prev = *arr.last().unwrap_or(&1);

    for k in prev..=sum {
        let mut next_arr = arr.to_owned();
        next_arr.push(k);

        if k <= reduced_num {
            sum_perms_helper(sum, reduced_num - k, &next_arr, res);
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
    sum_perms_helper(sum, sum, &arr, &mut res);
    res
}

/// Verify whether a point set is a valid strong orthogonal array based on the metadata supplied in
/// that struct. This method returns whether the given SOA is valid, based on the metadata. It will
/// check that the SOA maintains the stratification guarantees based on the properties of the SOA.
pub fn verify(soa: &SOA) -> bool {
    // The exponents for each strata. For example, [1, 1, 1] means s^1 x s^1 x s^1 strata
    let strata_exp = sum_perms(soa.strength);

    // In this loop, we test each combination of strata to ensure that the SOA can be
    // reduced down to some lower asymmetrical orthogonal array
    for curr_strata in strata_exp {
        // this yields every possible permutation of the strata exponents
        let strata_perms = curr_strata.iter().combinations(curr_strata.len());

        // For each permutation of strata, we have to try each permutation relative to each axis
        // For example, for s^2 x s, we check to see if dim 0 is stratified with s^2, and
        // dim 1 is stratified with s, then if dim 1 is stratified with s^2 and dim 0 with
        // s
        for strata_perm in strata_perms {
            // Generate a "ground-truth" set with the combinations we should see in the SOA
            // We set this up by doing a cartesian product over a range of vectors 0..s^pow
            // for each strata power value

            let expected_combos = strata_perm
                // note that we use `into_iter` rather than `iter` because we are already
                // referencing the strata permutation vector and there's no benefit to
                // getting a pointer to a pointer
                .iter()
                .map(|x| 0..soa.base.pow(**x))
                .multi_cartesian_product();

            let mut combo_counter: HashMap<Vec<u32>, u32> =
                expected_combos.map(|x| (x, 0)).collect();

            // for each combination of columns of size(strata_perm), check that the expected combos
            // match up with the actual combos when we "reduce" the OA to a lesser OA using the
            // method described by He and Tang (just divide by s^pow)
            // Every subset of g columns must be an OA of power g with uneven levels that were
            // determined by the power of the strata
            // We don't care how many instances of each pair are present because He and Tang define
            // an OA to be one with an arbitrary index (aka as long as we have the same number of
            // each tuple we're good to go), or lambda >= 1
            let column_combos = (0..soa.points.shape()[1]).combinations(strata_perm.len());

            for col_combo in column_combos {
                combo_counter = combo_counter.iter().map(|(k, _)| (k.clone(), 0)).collect();

                for row in soa.points.genrows() {
                    let mut point = Vec::new();

                    for (strata_pow, col) in zip(strata_perm.iter(), col_combo.iter()) {
                        point.push(row[[*col]] / soa.base.pow(soa.strength - **strata_pow));
                    }

                    // if the row of the reduced OA is not in the hash map, then there is an error
                    if !combo_counter.contains_key(&point) {
                        return false;
                    }
                    *combo_counter.entry(point).or_default() += 1;
                }

                // check that all entries have equal frequency and are greater than 0
                if combo_counter.values().any(|&x| x < 1) {
                    return false;
                }
                let uniq: HashSet<u32> = combo_counter.values().cloned().collect();

                if uniq.len() > 1 {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
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

    #[test]
    fn test_verify_valid_soa() {
        let ground_truth = array![
            [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [7, 6, 3, 6, 2, 2, 3, 7, 7, 6, 3],
            [5, 5, 4, 1, 4, 0, 0, 1, 5, 5, 5],
            [6, 3, 7, 6, 3, 6, 2, 2, 3, 7, 7],
            [7, 6, 3, 7, 6, 3, 6, 2, 2, 3, 7],
            [7, 7, 6, 3, 7, 6, 3, 6, 2, 2, 3],
            [5, 5, 5, 4, 1, 5, 4, 1, 4, 0, 1],
            [4, 1, 5, 5, 4, 1, 5, 4, 1, 4, 1],
            [4, 0, 1, 5, 5, 4, 1, 5, 4, 1, 5],
            [6, 2, 2, 3, 7, 7, 6, 3, 7, 6, 3],
            [5, 4, 0, 0, 1, 5, 5, 4, 1, 5, 5],
            [6, 3, 6, 2, 2, 3, 7, 7, 6, 3, 7],
            [3, 7, 7, 7, 7, 7, 7, 7, 7, 7, 6],
            [0, 1, 4, 1, 5, 5, 4, 0, 0, 1, 4],
            [2, 2, 3, 6, 3, 7, 7, 6, 2, 2, 2],
            [1, 4, 0, 1, 4, 1, 5, 5, 4, 0, 0],
            [0, 1, 4, 0, 1, 4, 1, 5, 5, 4, 0],
            [0, 0, 1, 4, 0, 1, 4, 1, 5, 5, 4],
            [2, 2, 2, 3, 6, 2, 3, 6, 3, 7, 6],
            [3, 6, 2, 2, 3, 6, 2, 3, 6, 3, 6],
            [3, 7, 6, 2, 2, 3, 6, 2, 3, 6, 2],
            [1, 5, 5, 4, 0, 0, 1, 4, 0, 1, 4],
            [2, 3, 7, 7, 6, 2, 2, 3, 6, 2, 2],
            [1, 4, 1, 5, 5, 4, 0, 0, 1, 4, 0],
        ];
        let soa = SOA {
            strength: 3,
            base: 2,
            points: ground_truth,
        };
        assert!(verify(&soa));

        // Taken from "A Characterization of Strong Orthogonal Arrays of Strength 3" (He and Tang,
        // 2014).
        let ground_truth = array![
            [0, 0, 0],
            [2, 3, 6],
            [3, 6, 2],
            [1, 5, 4],
            [6, 2, 3],
            [4, 1, 5],
            [5, 4, 1],
            [7, 7, 7],
        ];
        let soa = SOA {
            strength: 3,
            base: 2,
            points: ground_truth,
        };
        assert!(verify(&soa));
    }

    #[test]
    fn test_verify_invalid_soa() {
        let ground_truth = array![
            [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [7, 6, 3, 6, 2, 2, 3, 7, 7, 6, 3],
            [5, 5, 4, 1, 4, 0, 0, 1, 5, 5, 5],
            [6, 3, 7, 6, 3, 6, 2, 2, 3, 7, 7],
            [7, 6, 3, 7, 6, 3, 6, 2, 2, 3, 7],
            [7, 7, 6, 3, 7, 6, 3, 6, 2, 2, 3],
            [5, 5, 5, 4, 1, 5, 4, 1, 4, 0, 1],
            [4, 1, 5, 5, 4, 1, 5, 4, 1, 4, 1],
            [4, 0, 1, 5, 5, 4, 1, 5, 4, 1, 5],
            [6, 2, 2, 3, 7, 7, 6, 3, 7, 6, 3],
            [5, 4, 0, 0, 1, 5, 5, 4, 1, 5, 5],
            [6, 3, 6, 2, 2, 3, 7, 7, 6, 3, 7],
            [3, 7, 7, 7, 7, 7, 7, 7, 7, 7, 6],
            [0, 1, 4, 1, 5, 5, 4, 0, 0, 1, 4],
            [2, 2, 3, 6, 3, 7, 7, 6, 2, 2, 2],
            [1, 4, 0, 1, 4, 1, 5, 5, 4, 0, 0],
            [0, 1, 4, 0, 1, 4, 1, 5, 5, 4, 0],
            [0, 0, 1, 4, 0, 1, 4, 1, 5, 5, 4],
            [2, 2, 2, 3, 6, 2, 3, 6, 3, 7, 6],
            [3, 6, 2, 2, 3, 6, 2, 3, 6, 3, 6],
            [3, 7, 6, 2, 2, 3, 6, 2, 3, 6, 2],
            [3, 7, 6, 2, 2, 3, 6, 2, 3, 6, 2],
            [1, 5, 5, 4, 0, 0, 1, 4, 0, 1, 4],
            [2, 3, 7, 7, 6, 2, 2, 3, 6, 2, 2],
            [1, 4, 1, 5, 5, 4, 0, 0, 1, 4, 0],
        ];

        let soa = SOA {
            strength: 3,
            base: 2,
            points: ground_truth,
        };
        assert!(!verify(&soa));

        let ground_truth = array![
            [4, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
            [7, 6, 3, 6, 2, 2, 3, 7, 7, 6, 3],
            [5, 5, 4, 1, 4, 0, 0, 1, 5, 5, 5],
            [6, 3, 7, 6, 3, 6, 2, 2, 3, 7, 7],
            [7, 6, 3, 7, 6, 3, 6, 2, 2, 3, 7],
            [7, 7, 6, 3, 7, 6, 3, 6, 2, 2, 3],
            [5, 5, 5, 4, 1, 5, 4, 1, 4, 0, 1],
            [4, 1, 5, 5, 4, 1, 5, 4, 1, 4, 1],
            [4, 0, 1, 5, 5, 4, 1, 5, 4, 1, 5],
            [6, 2, 2, 3, 7, 7, 6, 3, 7, 6, 3],
            [5, 4, 0, 0, 1, 5, 5, 4, 1, 5, 5],
            [6, 3, 6, 2, 2, 3, 7, 7, 6, 3, 7],
            [3, 7, 7, 7, 7, 7, 7, 7, 7, 7, 6],
            [0, 1, 4, 1, 5, 5, 4, 0, 0, 1, 4],
            [2, 2, 3, 6, 3, 7, 7, 6, 2, 2, 2],
            [1, 4, 0, 1, 4, 1, 5, 5, 4, 0, 0],
            [0, 1, 4, 0, 1, 4, 1, 5, 5, 4, 0],
            [0, 0, 1, 4, 0, 1, 4, 1, 5, 5, 4],
            [2, 2, 2, 3, 6, 2, 3, 6, 3, 7, 6],
            [3, 6, 2, 2, 3, 6, 2, 3, 6, 3, 6],
            [3, 7, 6, 2, 2, 3, 6, 2, 3, 6, 2],
            [1, 5, 5, 4, 0, 0, 1, 4, 0, 1, 4],
            [2, 3, 7, 7, 6, 2, 2, 3, 6, 2, 2],
            [1, 4, 1, 5, 5, 4, 0, 0, 1, 4, 0],
        ];
        let soa = SOA {
            strength: 3,
            base: 2,
            points: ground_truth,
        };
        assert!(!verify(&soa));
    }
}
