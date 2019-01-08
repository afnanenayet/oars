//! Various constructors that implement the `OAConstructor` trait, and provide
//! methods to generate orthogonal arrays. Each struct contains the
//! configuration parameters necessary for the construction method, and the
//! `gen` method (from the trait) will construct an orthogonal array from the
//! given parameters.
//!
//! The description of the construction method will be with the struct that contains
//! the parameters.

use crate::oa::{OACErrorKind, OAConstructionError, OAConstructor, OAResult, OA};
use ndarray::Array2;
use primes::is_prime;

/// Generate an orthogonal array with any prime base and a strength between 2 and p + 1
///
/// The Bush construction technique, as described by Art Owen in his currently unpublished Monte
/// Carlo textbook. In Chapter 10.4, he describes the Bush construction technique.
pub struct Bush {
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: u32,

    /// The desired strength of the orthogonal array. It must be greater than or equal to 2.
    /// It must also be
    pub strength: u32,

    /// The dimensionality of the orthogonal array
    pub dimensions: u32,
}

impl Bush {
    /// Verify the parameters for Bush construction. This checks to see whether the prime base
    /// is valid and returns whether the parameters are correct.
    ///
    /// For the Bush construction, the strength, `s`, must be between 2 and `p + 1`.
    /// The resulting OA will have `p` levels, a strength of `t`, and `p^t` samples.
    fn verify_params(&self) -> bool {
        if !is_prime(u64::from(self.prime_base)) {
            return false;
        }

        if self.dimensions < 2 || self.dimensions > self.prime_base + 1 {
            return false;
        }
        true
    }
}

//impl OAConstructor for Bush {
//fn gen(&self) -> OA {
// TODO implement this!
//}
//}

/// Generate an orthogonal array with any prime base and a strength of 2
///
/// This technique was described by Art Owen in his Monte Carlo book in Chapter 10.4.
///
/// `prime_base` corresponds to $p$ in the literature. The number of total
/// points, or $n$ is $p^2$.
///
/// `dimensions` determines how many dimensions the resulting point set will
/// be. It must be between 2 and $p + 1$, inclusive.
pub struct Bose {
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: u32,

    /// The dimensionality of the orthogonal array
    pub dimensions: u32,
}

impl Bose {
    /// Verify the parameters for Bose construction and return whether they
    /// are valid.
    fn verify_params(&self) -> bool {
        if self.dimensions < 2 || self.dimensions > self.prime_base + 1 {
            return false;
        }

        if !is_prime(u64::from(self.prime_base)) {
            return false;
        }
        true
    }
}

impl OAConstructor for Bose {
    fn gen(&self) -> OAResult {
        if !self.verify_params() {
            return Err(OAConstructionError::new(
                OACErrorKind::InvalidParams,
                "invalid parameters",
            ));
        }

        let n = self.prime_base * self.prime_base;
        let mut points = Array2::<u32>::zeros((n as usize, self.dimensions as usize));

        // Initialize dims 1 and 2 with the special construction technique
        for i in 0..n {
            points[[i as usize, 0]] = i / self.prime_base;
            points[[i as usize, 1]] = i % self.prime_base;
        }

        for i in 0..n {
            for j in 2..self.dimensions {
                points[[i as usize, j as usize]] =
                    points[[i as usize, 1]] + (j - 2) * points[[i as usize, 2]];
            }
        }

        Ok(OA {
            strength: 2,
            levels: self.prime_base,
            factors: self.dimensions,
            index: 1,
            points,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
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
}
