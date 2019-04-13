//! Various constructors that implement the `OAConstructor` trait, and provide
//! methods to generate orthogonal arrays. Each struct contains the
//! configuration parameters necessary for the construction method, and the
//! `gen` method (from the trait) will construct an orthogonal array from the
//! given parameters.
//!
//! The description of the construction method will be with the struct that contains
//! the parameters.

use crate::oa::{OACErrorKind, OAConstructionError, OAConstructor, OAInteger, OAResult, OA};
use crate::utils::{poly_eval, to_base_fixed};
use ndarray::Array2;
use num::pow::pow;
use primes::is_prime;
use std::cmp::min;

/// Generate an orthogonal array with any prime base and a strength between 2 and p + 1
///
/// The Bush construction technique, as described by Art Owen in his currently unpublished Monte
/// Carlo textbook. In Chapter 10.4, he describes the Bush construction technique.
pub struct Bush<T: OAInteger> {
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: T,

    /// The desired strength of the orthogonal array. It must be greater than or equal to 2.
    /// It must also be
    pub strength: T,

    /// The dimensionality of the orthogonal array
    pub dimensions: T,
}

impl<T: OAInteger> Bush<T> {
    /// Verify the parameters for Bush construction. This checks to see whether the prime base
    /// is valid and returns whether the parameters are correct.
    ///
    /// For the Bush construction, the strength, `s`, must be between 2 and `p + 1`.
    /// The resulting OA will have `p` levels, a strength of `t`, and `p^t` samples.
    fn verify_params(&self) -> bool {
        if !is_prime(self.prime_base.to_u64().unwrap()) {
            return false;
        }

        if self.dimensions < T::from(2).unwrap()
            || self.dimensions > self.prime_base + T::from(1).unwrap()
        {
            return false;
        }

        if self.strength < T::from(1).unwrap() || self.strength > self.prime_base {
            return false;
        }
        true
    }
}

impl<T: OAInteger> OAConstructor<T> for Bush<T> where {
    fn gen(&self) -> OAResult<T> {
        if !self.verify_params() {
            return Err(OAConstructionError::new(
                OACErrorKind::InvalidParams,
                "Invalid parameters",
            ));
        }
        let n = pow(self.prime_base, self.strength.to_usize().unwrap());
        let mut points =
            Array2::<T>::zeros((n.to_usize().unwrap(), self.dimensions.to_usize().unwrap()));

        for i in 0..n.to_usize().unwrap() {
            let coeffs = to_base_fixed(T::from(i).unwrap(), self.prime_base, self.strength);
            let poly_dims = min(self.dimensions, self.prime_base);

            for j in 0..poly_dims.to_usize().unwrap() {
                points[[i as usize, j as usize]] =
                    poly_eval(&coeffs, T::from(j).unwrap()) % self.prime_base;
            }

            if self.dimensions == self.prime_base + T::from(1).unwrap() {
                points[[i, self.prime_base.to_usize().unwrap()]] =
                    T::from(i - 1).unwrap() % self.prime_base;
            }
        }

        Ok(OA {
            strength: self.strength,
            levels: self.prime_base,
            index: T::from(1).unwrap(),
            factors: self.dimensions,
            points,
        })
    }
}

/// Generate an orthogonal array with any prime base and a strength of 2
///
/// This technique was described by Art Owen in his Monte Carlo book in Chapter 10.4.
///
/// `prime_base` corresponds to $p$ in the literature. The number of total
/// points, or $n$ is $p^2$.
///
/// `dimensions` determines how many dimensions the resulting point set will
/// be. It must be between 2 and $p + 1$, inclusive.
pub struct Bose<T: OAInteger> {
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: T,

    /// The dimensionality of the orthogonal array
    pub dimensions: T,
}

impl<T: OAInteger> Bose<T> {
    /// Verify the parameters for Bose construction and return whether they
    /// are valid.
    fn verify_params(&self) -> bool {
        if self.dimensions < T::from(2).unwrap()
            || self.dimensions > self.prime_base + T::from(1).unwrap()
        {
            return false;
        }

        if !is_prime(self.prime_base.to_u64().unwrap()) {
            return false;
        }
        true
    }
}

impl<T: OAInteger> OAConstructor<T> for Bose<T> {
    fn gen(&self) -> OAResult<T> {
        if !self.verify_params() {
            return Err(OAConstructionError::new(
                OACErrorKind::InvalidParams,
                "invalid parameters",
            ));
        }
        let n = pow(self.prime_base, 2);
        let mut points =
            Array2::<T>::zeros((n.to_usize().unwrap(), self.dimensions.to_usize().unwrap()));

        // Initialize dims 1 and 2 with the special construction technique
        for i in 0..n.to_usize().unwrap() {
            points[[i as usize, 0]] = T::from(i).unwrap() / self.prime_base;
            points[[i as usize, 1]] = T::from(i).unwrap() % self.prime_base;
        }

        for i in 0..n.to_usize().unwrap() {
            for j in 2..self.dimensions.to_usize().unwrap() {
                points[[i, j]] = (points[[i, 0]]
                    + T::from(j - 1).unwrap() * points[[i as usize, 1]])
                    % self.prime_base;
            }
        }

        Ok(OA {
            strength: T::from(2).unwrap(),
            levels: self.prime_base,
            factors: self.dimensions,
            index: T::from(1).unwrap(),
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
        let bose: Bose<u32> = Bose {
            prime_base: u32::From(4),
            dimensions: u32::From(3),
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
            [1, 0, 1],
            [1, 1, 2],
            [1, 2, 0],
            [2, 0, 2],
            [2, 1, 0],
            [2, 2, 1],
        ]);
        println!("{:?}", oa.points);
        assert!(oa.points == ground_truth);
    }

    #[test]
    fn bush_non_prime() {
        let bush = Bush {
            strength: 2,
            prime_base: 4,
            dimensions: 3,
        };
        assert!(bush.gen().is_err());

        let bush = Bush {
            strength: 2,
            prime_base: 9,
            dimensions: 3,
        };
        assert!(bush.gen().is_err());

        let bush = Bush {
            strength: 2,
            prime_base: 100,
            dimensions: 3,
        };
        assert!(bush.gen().is_err());
    }
}
