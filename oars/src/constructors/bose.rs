use crate::oa::{OAConstructor, OAResult, OA};
use crate::utils::{Integer, OarsError, OarsResult};
use ndarray::Array2;
use num::pow;
use oars_proc_macro::Checked;
use primes::is_prime;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
use crate::oa::ParOAConstructor;

#[cfg(feature = "parallel")]
use ndarray::{concatenate, Axis};

impl<T: Integer> BoseChecked<T> {
    /// Check the parameters for Bose construction
    ///
    /// This method ensures that the parameters for Bose construction are valid. If they are not,
    /// this will return an error. Upon success, the `BoseChecked` struct is consumed, and a `Bose`
    /// struct is returned, which implemented the OA generation methods.
    ///
    /// ```
    /// use oars::prelude::*;
    /// # fn main() -> OarsResult<()> {
    /// use oars::constructors::{BoseChecked, Bose};
    ///
    /// let bose = BoseChecked {
    ///     prime_base: 3,
    ///     dimensions: 2,
    /// };
    ///
    /// // On success, the BoseChecked struct is consumed and replaced with a Bose struct that is
    /// // ready to generate an OA
    /// let oa = bose.verify()?.gen();
    /// # Ok(())
    /// # }
    /// ```
    pub fn verify(self) -> OarsResult<Bose<T>> {
        if self.dimensions < T::from(2).unwrap()
            || self.dimensions > self.prime_base + T::from(1).unwrap()
        {
            return Err(OarsError::InvalidParams("Invalid dimensions".into()));
        }

        if !is_prime(self.prime_base.to_u64().unwrap()) {
            return Err(OarsError::InvalidParams("Base is not prime".into()));
        }
        Ok(Bose {
            prime_base: self.prime_base,
            dimensions: self.dimensions,
        })
    }
}

/// Generate an orthogonal array with any prime base and a strength of 2
///
///
/// This technique was described by Art Owen in his Monte Carlo book in Chapter 10.4.
///
/// `prime_base` corresponds to $p$ in the literature. The number of total
/// points, or $n$ is $p^2$.
///
/// `dimensions` determines how many dimensions the resulting point set will
/// be. It must be between 2 and $p + 1$, inclusive.
///
/// Note that using this struct directly does not offer parameter checking, so the user should be
/// sure that the supplied parameters are valid. If invalid parameters are supplied, then the
/// resultant orthogonal array with be malformed and invalid. To utilize parameter checking,
/// construct a `BoseChecked` struct instead.
#[derive(Checked)]
pub struct Bose<T: Integer> {
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: T,

    /// The dimensionality of the orthogonal array
    pub dimensions: T,
}

impl<T: Integer> OAConstructor<T> for Bose<T> {
    fn gen(&self) -> OAResult<T> {
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
impl<T: Integer> ParOAConstructor<T> for Bose<T> {
    fn gen_par(&self) -> OAResult<T> {
        let n = pow(self.prime_base, 2);

        // We create two different arrays: the first two columns and the rest, because the latter
        // is dependent on the first, so each array is constructed in parallel and then
        // concatenated
        let mut initial_points = Array2::<T>::zeros((n.to_usize().unwrap(), 2));
        let mut points = Array2::<T>::zeros((
            n.to_usize().unwrap(),
            self.dimensions.to_usize().unwrap() - 2,
        ));

        // Initialize the first two dimensions first, since all subsequent dimensions depend on the
        // these dims
        initial_points
            .axis_iter_mut(Axis(1))
            .into_par_iter()
            .enumerate()
            .for_each(|(col_idx, mut col)| {
                col.axis_iter_mut(Axis(0))
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(row_idx, mut row)| match col_idx {
                        0 => row[[row_idx; 0]] = T::from(row_idx).unwrap() / self.prime_base,
                        1 => row[[row_idx; 0]] = T::from(row_idx).unwrap() % self.prime_base,
                        _ => panic!("A column besides 0 or 1 was reached, which is impossible"),
                    })
            });

        // every remaining point can be calculated independently, so we separate them out into a
        // different threadpool
        points
            .axis_iter_mut(Axis(1))
            .into_par_iter()
            .enumerate()
            .for_each(|(col_idx, mut col)| {
                col.axis_iter_mut(Axis(0))
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(row_idx, mut row)| {
                        row[[row_idx; 0]] = (initial_points[[row_idx, 0]]
                            + T::from(col_idx + 1).unwrap() * initial_points[[row_idx, 1]])
                            % self.prime_base;
                    })
            });
        let points: Array2<T> = concatenate(Axis(1), &[initial_points.view(), points.view()])?;
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
    #[cfg(feature = "parallel")]
    fn bose_par_init_2() {
        let bose = Bose {
            prime_base: 2,
            dimensions: 2,
        };
        let oa = bose.gen_par().unwrap();
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
        assert!(oa.points == ground_truth);
    }
}
