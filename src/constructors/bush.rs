use crate::oa::{OAConstructor, OAResult, OA};
use crate::utils::{poly_eval, to_base_fixed, ErrorKind, Integer, OarsError, OarsResult};
use ndarray::Array2;
use num::pow::pow;
use primes::is_prime;
use std::cmp::min;

#[cfg(feature = "parallel")]
use crate::oa::ParOAConstructor;

#[cfg(feature = "parallel")]
use ndarray::{stack, Axis};

#[cfg(feature = "parallel")]
use ndarray_parallel::prelude::*;

/// Generate an orthogonal array with any prime base and a strength between 2 and p + 1 with
/// parameter checking.
///
/// The Bush construction technique, as described by Art Owen in his currently unpublished Monte
/// Carlo textbook. In Chapter 10.4, he describes the Bush construction technique.
///
/// This struct can not generate orthogonal arrays, as it represents a pre-verified state that must
/// be consumed before generating OAs.
pub struct BushChecked<T: Integer> {
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: T,

    /// The desired strength of the orthogonal array. It must be greater than or equal to 2.
    /// It must also be
    pub strength: T,

    /// The dimensionality of the orthogonal array
    pub dimensions: T,
}

impl<T: Integer> BushChecked<T> {
    /// Verify that the parameters for Bush construction are valid
    ///
    /// This method returns a `Bush` struct upon success and consumes the original struct. If there
    /// is an error, this will return an `OarsError` and consume the original struct.
    ///
    /// Example usage:
    ///
    /// ```
    /// use oars::prelude::*;
    /// use oars::constructors::{Bush, BushChecked};
    /// # fn main() -> OarsResult<()> {
    /// let bush = BushChecked {
    ///     prime_base: 5,
    ///     dimensions: 3,
    ///     strength: 3,
    /// };
    /// let oa = bush.verify()?.gen();
    /// # Ok(())
    /// # }
    /// ```
    pub fn verify(self) -> OarsResult<Bush<T>> {
        if !is_prime(self.prime_base.to_u64().unwrap()) {
            return Err(OarsError::new(
                ErrorKind::InvalidParams,
                "Base is not prime",
            ));
        }

        if self.dimensions < T::from(2).unwrap()
            || self.dimensions > self.prime_base + T::from(1).unwrap()
        {
            return Err(OarsError::new(
                ErrorKind::InvalidParams,
                "Dimensions must be less than `prime_base` + 1",
            ));
        }

        if self.strength < T::from(1).unwrap() || self.strength > self.prime_base {
            return Err(OarsError::new(
                ErrorKind::InvalidParams,
                "`strength` must be between 1 and `prime_base` (inclusive)",
            ));
        }
        Ok(Bush {
            strength: self.strength,
            prime_base: self.prime_base,
            dimensions: self.dimensions,
        })
    }
}

/// Generate an orthogonal array with any prime base and a strength between 2 and p + 1
///
/// The Bush construction technique, as described by Art Owen in his currently unpublished Monte
/// Carlo textbook. In Chapter 10.4, he describes the Bush construction technique.
///
/// Note that using this struct directly does not check any parameters. You should only use
/// this if you are certain that your parameters are valid, otherwise the resulting orthogonal
/// array will be invalid.
pub struct Bush<T: Integer> {
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: T,

    /// The desired strength of the orthogonal array. It must be greater than or equal to 2.
    /// It must also be
    pub strength: T,

    /// The dimensionality of the orthogonal array
    pub dimensions: T,
}

impl<T: Integer> OAConstructor<T> for Bush<T> {
    fn gen(&self) -> OAResult<T> {
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

#[cfg(feature = "parallel")]
impl<T: Integer> ParOAConstructor<T> for Bush<T> {
    fn gen_par(&self) -> OAResult<T> {
        let n = pow(self.prime_base, self.strength.to_usize().unwrap());

        let mut initial_points = Array2::<T>::zeros((
            n.to_usize().unwrap(),
            min(
                self.dimensions.to_usize().unwrap(),
                self.prime_base.to_usize().unwrap(),
            ),
        ));

        initial_points
            .axis_iter_mut(Axis(0))
            .into_par_iter()
            .enumerate()
            .for_each(|(row_idx, mut row)| {
                let coeffs =
                    to_base_fixed(T::from(row_idx).unwrap(), self.prime_base, self.strength);
                row.axis_iter_mut(Axis(0))
                    .into_iter()
                    .enumerate()
                    .for_each(|(col_idx, mut col)| {
                        col[[col_idx; 0]] =
                            poly_eval(&coeffs, T::from(col_idx).unwrap()) % self.prime_base;
                    })
            });

        // There is a special case for the last column if it is equal to prime_base + 1, which is
        // documented by Art Owen. We take care of this special case here, because otherwise it's
        // an unnecessary calculation.
        if self.dimensions == self.prime_base + T::from(1).unwrap() {
            let mut last_col = Array2::<T>::zeros((n.to_usize().unwrap(), 1));

            last_col
                .axis_iter_mut(Axis(0))
                .into_par_iter()
                .enumerate()
                .for_each(|(row_idx, mut row)| {
                    row.axis_iter_mut(Axis(0))
                        .into_iter()
                        .enumerate()
                        .for_each(|(_, mut col)| {
                            col[[0 as usize; 0]] = T::from(row_idx - 1).unwrap() % self.prime_base;
                        })
                });
            let points = stack![Axis(1), initial_points, last_col];

            return Ok(OA {
                strength: self.strength,
                levels: self.prime_base,
                index: T::from(1).unwrap(),
                factors: self.dimensions,
                points,
            });
        }

        Ok(OA {
            strength: self.strength,
            levels: self.prime_base,
            index: T::from(1).unwrap(),
            factors: self.dimensions,
            points: initial_points,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bush_non_prime() {
        let bush = BushChecked {
            strength: 2,
            prime_base: 4,
            dimensions: 3,
        };
        assert!(bush.verify().is_err());

        let bush = BushChecked {
            strength: 2,
            prime_base: 9,
            dimensions: 3,
        };
        assert!(bush.verify().is_err());

        let bush = BushChecked {
            strength: 2,
            prime_base: 100,
            dimensions: 3,
        };
        assert!(bush.verify().is_err());
    }

    #[test]
    fn bush_bad_dims() {
        let bush = BushChecked {
            strength: 2,
            prime_base: 5,
            dimensions: 7,
        };
        assert!(bush.verify().is_err());

        let bush = BushChecked {
            strength: 2,
            prime_base: 7,
            dimensions: 11,
        };
        assert!(bush.verify().is_err());

        let bush = BushChecked {
            strength: 2,
            prime_base: 13,
            dimensions: 17,
        };
        assert!(bush.verify().is_err());
    }
}
