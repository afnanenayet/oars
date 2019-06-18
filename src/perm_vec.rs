//! Provides an interface for a permutation vector class that makes it easy to
//! randomly shuffle orthogonal arrays, or shuffle any set.

use rand;
use rand::seq::SliceRandom;
use std::ops::Index;

/// This is a vector containing the elements ${0, 1 \cdots n - 1}$, shuffled
/// around to easily allow for a user to create a permutation.
///
/// For example: `a[0]`, if `a` is shuffled, corresponds to another element.
/// This allows you to perform an O(1) permutation as long as `a` has been
/// shuffled.
#[derive(Debug)]
pub struct PermutationVector {
    /// The internal array containing the shuffled elements.
    vec: Vec<usize>,
}

impl Index<usize> for PermutationVector {
    type Output = usize;

    fn index(&self, index: usize) -> &usize {
        &self.vec[index]
    }
}

impl PermutationVector {
    /// Create a permutation vector that has not been shuffled. If you want a random permutation,
    /// you must remember to call the `shuffle()` method.
    pub fn new(n: usize) -> Self {
        Self {
            vec: (0..n).collect(),
        }
    }

    /// Randomly shuffle the permutation vector
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.vec.shuffle(&mut rng);
    }
}
