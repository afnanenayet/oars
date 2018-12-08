//! The generic interface to define an orthogonal array and generic construction methods. This
//! module also defines a few construction methods.

use crate::perm_vec::PermutationVector;
use ndarray::Array2;
use rand::prelude::*;
use std::fmt;

/// The definition of an orthogonal array with its point set and parameters.
#[derive(Debug)]
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
            "OA:\n\tlevels: {}\n\tstrength: {}\n\tfactors: {}\n\tindex: {}\npoints:\n{}\n",
            self.levels, self.strength, self.factors, self.index, self.points
        )
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
        perms[i] = PermutationVector::new(dims[0]);

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

/// A generic trait to demarcate orthogonal array constructors
pub trait OAConstructor {
    /// The method that generates an orthogonal array. Any necessary parameters must be handled
    /// by the constructor itself.
    fn gen(&self) -> OA;
}
