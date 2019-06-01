use crate::oa::{OACErrorKind, OAConstructionError, OAConstructor, OAResult, OA};
use ndarray::{Array2, Axis};
use num::{Integer, NumCast};
use primes::is_prime;
use num::pow;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
use ndarray_parallel::prelude::*;

#[cfg(feature = "parallel")]
use crate::oa::ParOAConstructor;

/// Generate an orthogonal array with any prime base and a strength of 2
///
/// This technique was described by Art Owen in his Monte Carlo book in Chapter 10.4.
///
/// `prime_base` corresponds to $p$ in the literature. The number of total
/// points, or $n$ is $p^2$.
///
/// `dimensions` determines how many dimensions the resulting point set will
/// be. It must be between 2 and $p + 1$, inclusive.
pub struct Bose<T>
where
    T: NumCast + Integer + Copy,
{
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: T,

    /// The dimensionality of the orthogonal array
    pub dimensions: T,
}

impl<T> Bose<T>
where
    T: NumCast + Integer + Copy,
{
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

impl<T> OAConstructor<T> for Bose<T>
where
    T: NumCast + Integer + Copy,
{
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

#[cfg(feature = "parallel")]
impl<T> ParOAConstructor<T> for Bose<T>
where
    T: NumCast + Integer + Copy,
{
    fn gen_par(&self) -> OAResult<T> {
        if !self.verify_params() {
            return Err(OAConstructionError::new(
                OACErrorKind::InvalidParams,
                "invalid parameters",
            ));
        }
        let n = pow(self.prime_base, 2);
        let mut points =
            Array2::<T>::zeros((n.to_usize().unwrap(), self.dimensions.to_usize().unwrap()));

        // Initialize the first two dimensions first, since all subsequent dimensions depend on the
        // these dims
        points
            .axis_iter_mut(Axis(0))
            .into_par_iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.par_iter_mut()
                    .take(2)
                    .enumerate(|(col_idx, val)| match col_idx {
                        0 => T::from(row_idx).unwrap() / self.prime_base,
                        1 => T::from(row_idx).unwrap() % self.prime_base,
                    })
            });

        // every remaining point can be calculated independently, so we separate them out into a
        // different threadpool
        points
            .axis_iter(Axis(0))
            .par_iter_mut()
            .enumerate(|(row_idx, row)| {
                row.par_iter_mut().skip(2).enumerate(|(col_idx, val)| {
                    points[[row_idx, 0]]
                        + T::from(col_idx - 1).unwrap() * points[[row_idx as usize, 1]]
                            % self.prime_base
                })
            });
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
        let bose = Bose {
            prime_base: 4,
            dimensions: 4,
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

}
