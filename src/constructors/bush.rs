use crate::oa::{OACErrorKind, OAConstructionError, OAConstructor, OAResult, OA};
use crate::utils::{poly_eval, to_base_fixed, Integer};
use ndarray::Array2;
use num::pow::pow;
use primes::is_prime;
use std::cmp::min;

/// Generate an orthogonal array with any prime base and a strength between 2 and p + 1
///
/// The Bush construction technique, as described by Art Owen in his currently unpublished Monte
/// Carlo textbook. In Chapter 10.4, he describes the Bush construction technique.
pub struct Bush<T: Integer>
{
    /// The strength of the orthogonal array. It *must* be a prime number.
    pub prime_base: T,

    /// The desired strength of the orthogonal array. It must be greater than or equal to 2.
    /// It must also be
    pub strength: T,

    /// The dimensionality of the orthogonal array
    pub dimensions: T,
}

impl<T: Integer> Bush<T>
{
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

impl<T: Integer> OAConstructor<T> for Bush<T>
{
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

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

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
