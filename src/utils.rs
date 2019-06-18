//! Misc utilities and convenience functions for the library

use num::{self, NumCast};
use std::vec::Vec;

/// A generic integer type that is compatible with the orthogonal array and strong orthogonal array
/// definitions in the library. Almost every type that is compatible with `num::Integer` from the
/// num-traits crate should be compatible with this integer type.
///
/// This trait is deprecated, and the [Integer](oars::Integer) trait should be used instead.
#[deprecated(since = "0.3.0", note = "Use oars::Integer instead")]
pub trait OAInteger: NumCast + num::Integer + Copy {}

#[allow(deprecated)]
impl<T> OAInteger for T where T: NumCast + num::Integer + Copy {}

/// A generic float type that is compatible with the orthogonal array and strong orthogonal array
/// definitions in the library. Almost every type that is compatible with `num::Float` from the
/// num-traits crate should be compatible with this float type.
///
/// This trait is deprecated, and the [Float](oars::Float) trait should be used instead.
#[deprecated(since = "0.3.0", note = "Use oars::Float instead")]
pub trait OAFloat: NumCast + num::Float + Copy {}

#[allow(deprecated)]
impl<T> OAFloat for T where T: NumCast + num::Float + Copy {}

/// A generic integer type.
///
/// A generic integer type that is compatible with the orthogonal array and strong orthogonal array
/// definitions in the library. Almost every type that is compatible with `num::Integer` from the
/// num-traits crate should be compatible with this integer type.
pub trait Integer: NumCast + num::Integer + Copy {}
impl<T> Integer for T where T: NumCast + num::Integer + Copy {}

/// A generic floating point type.
///
/// A generic float type that is compatible with the orthogonal array and strong orthogonal array
/// definitions in the library. Almost every type that is compatible with `num::Float` from the
/// num-traits crate should be compatible with this float type.
pub trait Float: NumCast + num::Float + Copy {}
impl<T> Float for T where T: NumCast + num::Float + Copy {}

/// Convert a number to an arbitrary base with a fixed number of digits
///
/// Given some number, convert the number to some base with a specified number of digits. This
/// means that numbers can be truncated if `degree` is too small. This also means that numbers may
/// be zero-padded.
pub fn to_base_fixed<T: Integer>(num: T, base: T, degree: T) -> Vec<T>
where
{
    // The number in a the new base
    let mut new_base = vec![T::from(0).unwrap(); degree.to_usize().unwrap()];
    let mut new_num = num;

    for i in 0..degree.to_usize().unwrap() {
        let i = i as usize;
        new_base[i] = new_num % base;
        new_num = new_num / base;
    }
    new_base
}

/// Evaluate a number in some base representation in base 10.
///
/// Given some vector of coefficients, which represent a number in some arbitrary base
/// representation, convert the number to a base 10 representation using Horner's rule.
pub fn poly_eval<T>(coeffs: &[T], base: T) -> T
where
    T: Integer + NumCast + Copy,
{
    let mut result: T = T::from(0).unwrap();

    // Using Horner's rule
    for coefficient in coeffs.iter().rev() {
        result = (result * base) + *coefficient;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base_fixed() {
        let num = 5;
        let base = 2;
        let mut res = to_base_fixed(num, base, 3);
        res.reverse();
        assert!(res == vec![1, 0, 1]);

        let num = 1;
        let base = 2;
        let mut res = to_base_fixed(num, base, 3);
        res.reverse();
        assert!(res == vec![0, 0, 1]);

        let num = 9;
        let base = 2;
        let mut res = to_base_fixed(num, base, 5);
        res.reverse();
        assert!(res == vec![0, 1, 0, 0, 1]);

        let num = 7;
        let base = 3;
        let mut res = to_base_fixed(num, base, 2);
        res.reverse();
        assert!(res == vec![2, 1]);
    }

    #[test]
    fn test_poly_eval() {
        let coeffs = vec![1, 1, 1, 1];
        let base = 2;
        let result = poly_eval(&coeffs, base);
        assert!(result == 15);

        let coeffs = vec![1, 2];
        let base = 3;
        let result = poly_eval(&coeffs, base);
        assert!(result == 7);

        let coeffs = vec![1, 0, 0, 1];
        let base = 2;
        let result = poly_eval(&coeffs, base);
        assert!(result == 9);

        let coeffs = vec![0, 0, 1];
        let base = 3;
        let result = poly_eval(&coeffs, base);
        assert!(result == 9);
    }
}
