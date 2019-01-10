//! The generic interface to define an orthogonal array and generic construction methods. This
//! module also defines a few construction methods.

use crate::perm_vec::PermutationVector;
use itertools::Itertools;
use ndarray::Array2;
use rand::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// The definition of an orthogonal array with its point set and parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct OA {
    /// The size of the set $X$ that the array can select elements from.
    pub levels: u32,

    /// The size of the t-tuple. In other words, this is the dimensionality of the stratification
    /// guarantee.
    pub strength: u32,

    /// The number of columns in the orthogonal array. This is the dimensionality of the point
    /// set.
    pub factors: u32,

    /// The number of times each t-tuple is present in the orthogonal array. Setting this to 1
    /// ensures the Latin hypercube guarantee.
    pub index: u32,

    /// The internal array that holds the data for the orthogonal array. This is not the same as
    /// the point set that can be used for Monte Carlo simulations.
    pub points: Array2<u32>,
}

/// Print the metadata of the orthogonal array, then print the contents of the array.
impl fmt::Display for OA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OA:\n\tlevels: {}\n\tstrength: {}\n\tfactors: {}\n\tindex: {}\npoints:\n\n{}\n\n",
            self.levels, self.strength, self.factors, self.index, self.points
        )
    }
}

/// The general categories of errors for `OAConstructionError`
#[derive(Debug)]
pub enum OACErrorKind {
    /// Invalid parameters were supplied to the constructor
    InvalidParams,

    /// There was a runtime error that prevented the orthogonal array from being properly
    /// constructed
    RuntimeError,
}

/// An error indicating that there was some error constructing the orthogonal array.
#[derive(Debug)]
pub struct OAConstructionError {
    /// The general category of the error
    error_type: OACErrorKind,

    /// A user-friendly description of the array which can supply additional information about
    /// the error.
    desc: String,
}

/// A result type for orthogonal array construction
pub type OAResult = Result<OA, OAConstructionError>;

impl Error for OAConstructionError {
    fn description(&self) -> &str {
        &self.desc
    }
}

impl fmt::Display for OAConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OA Construction Error: {}", &self.desc)
    }
}

impl OAConstructionError {
    pub fn new<T>(kind: OACErrorKind, msg: T) -> OAConstructionError
    where
        T: Into<String>,
    {
        OAConstructionError {
            error_type: kind,
            desc: msg.into(),
        }
    }
}

/// Normalize an orthogonal array into a point set using Art Owen's normalization technique.
/// This method takes a regular orthogonal array, and converts it into a point set in the $[0, 1)^m$
/// domain, so that it can be used as a sampling point set for Monte Carlo integration.
///
/// _Note: it is unknown whether this method will work with construction techniques other than Bose
/// and Bush construction._
///
/// Args:
///     - jitter: The factor between 0 and 1 to jitter by.
///     - randomize: Whether the orthogonal array should be randomly shuffled when generating points.
pub fn normalize(oa: &OA, jitter: f32, randomize: bool) -> Array2<f32> {
    if oa.points.ndim() != 2 {
        panic!("Orthogonal array must be in a 2D matrix form");
    }

    if jitter < 0.0 || jitter > 1.0 {
        panic!("Jitter factor must be between 0.0 and 1.0 (inclusive)");
    }

    let dims = oa.points.shape();
    let mut point_set = Array2::<f32>::zeros((dims[0], dims[1]));

    let mut perms: Vec<PermutationVector> = Vec::new();
    let mut rng = rand::thread_rng();

    // Create the permutation vectors. If "randomize" is requested, apply the
    // shuffle. Otherwise, it will be an identity vector, and applying it will
    // not result in any randomization.
    for i in 0..dims[1] {
        perms.push(PermutationVector::new(dims[0]));

        if randomize {
            perms[i].shuffle();
        }
    }

    // loop through each point in the OA and convert to a point in the pointset
    // note: `genrows()` does not seem to implement `enumerate()` so we need the explicit loop
    // counter.
    //for row in oa.points.genrows() {
    for i in 0..dims[0] {
        for j in 0..dims[1] {
            // Apply the shuffle with the permutation vector to get the new index for the
            // point
            let shuffled_i = perms[j][i];

            // Apply jitter factor (random number between 0 and jitter as an upper bound)
            // If jitter is 0, then the points will be centered in the strata.
            let jittered_point: f32 = (oa.points[[i, j]] as f32) + (jitter * rng.gen::<f32>());
            point_set[[shuffled_i, j]] = jittered_point / oa.strength as f32;
        }
    }
    point_set
}

/// Given some orthogonal array struct, verify that the points are a valid orthogonal array as
/// described by the parameters.
///
/// An orthogonal array is defined by four key parameters, and this function attempts to see
/// if `points` matches up with the other parameters. This means that for any (and every)
/// selection of $t$ columns, every possible combination of $t$-tuples must be present in that
/// submatrix. You can easily map the combinations in a unique way using base $s$ where $s$ is
/// the number of factors in the array (assuming it is a symmetrical array).
pub fn verify_oa(oa: &OA) -> bool {
    if oa.points.ndim() != 2 {
        return false;
    }

    if oa.points.shape()[1] != oa.factors as usize {
        return false;
    }

    let col_combos = (0..oa.factors).combinations(oa.strength as usize);

    // this iterator gives us every possible combination of columns
    for selection in col_combos {
        // tuple count holds the count for how many times each possible tuple is seen
        let mut tuple_count: HashMap<u32, u32> = HashMap::new();

        // loop through the points and count up how many times we encounter the tuple
        for i in 0..oa.points.shape()[0] {
            let mut tuple_index = 0;

            for (pow, column) in selection.iter().enumerate() {
                tuple_index +=
                    oa.points[[i as usize, *column as usize]] * oa.levels.pow(pow as u32);
            }
            // set count to 1 if it doesn't exist, otherwise update the count
            *tuple_count.entry(tuple_index).or_insert(0) += 1;
        }

        // now verify that the hashmap has every possible combination, `index` times
        for i in 0..oa.levels.pow(oa.strength) {
            // if the entry is not present in the array, set the count to 0
            if *tuple_count.entry(i).or_insert(0) != oa.index {
                return false;
            }
        }
    }
    true
}

/// A generic trait to demarcate orthogonal array constructors
pub trait OAConstructor {
    /// The method that generates an orthogonal array. Any necessary parameters must be handled
    /// by the constructor itself.
    fn gen(&self) -> OAResult;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_verify_oa_bad_in() {
        let points = arr2(&[
            [0, 0, 0],
            [0, 1, 1],
            [0, 2, 2],
            [1, 0, 0],
            [1, 0, 0],
            [1, 2, 2],
            [2, 0, 0],
            [2, 1, 1],
            [2, 2, 2],
        ]);
        let oa = OA {
            strength: 3,
            levels: 3,
            index: 1,
            factors: 3,
            points,
        };
        assert!(!verify_oa(&oa));
    }

    #[test]
    fn test_verify_oa_good_in() {
        let points = arr2(&[
            [0, 0, 0],
            [0, 1, 1],
            [0, 2, 2],
            [1, 0, 1],
            [1, 1, 2],
            [1, 2, 0],
            [2, 0, 2],
            [2, 1, 0],
            [2, 2, 1],
        ]);
        let oa = OA {
            strength: 2,
            levels: 3,
            index: 1,
            factors: 3,
            points,
        };
        assert!(verify_oa(&oa));
    }
}
