//! Misc utilities and convenience functions for the library

use num::{self, NumCast};
use std::error::Error;
use std::fmt;
use std::vec::Vec;

/// A generic integer type.
///
/// A generic integer type that is compatible with the orthogonal array and strong orthogonal array
/// definitions in the library. Almost every type that is compatible with `num::Integer` from the
/// num-traits crate should be compatible with this integer type.
#[cfg(not(feature = "parallel"))]
pub trait Integer: NumCast + num::Integer + Copy {}

#[cfg(not(feature = "parallel"))]
impl<T> Integer for T where T: NumCast + num::Integer + Copy {}

#[cfg(feature = "parallel")]
pub trait Integer: NumCast + num::Integer + Copy + std::marker::Send + Sync {}

#[cfg(feature = "parallel")]
impl<T> Integer for T where T: NumCast + num::Integer + Copy + std::marker::Send + Sync {}

/// A generic floating point type.
///
/// A generic float type that is compatible with the orthogonal array and strong orthogonal array
/// definitions in the library. Almost every type that is compatible with `num::Float` from the
/// num-traits crate should be compatible with this float type.
#[cfg(not(feature = "parallel"))]
pub trait Float: NumCast + num::Float + Copy {}

#[cfg(not(feature = "parallel"))]
impl<T> Float for T where T: NumCast + num::Float + Copy {}

#[cfg(feature = "parallel")]
pub trait Float: NumCast + num::Float + Copy + std::marker::Send + Sync {}

#[cfg(feature = "parallel")]
impl<T> Float for T where T: NumCast + num::Float + Copy + std::marker::Send + Sync {}

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

/// The general categories of errors for `OAConstructionError`
#[derive(Debug)]
pub enum ErrorKind {
    /// Invalid parameters were supplied to the constructor
    InvalidParams,

    /// There was a runtime error that prevented the orthogonal array from being properly
    /// constructed
    RuntimeError,
}

/// An error indicating that there was some error constructing the orthogonal array.
#[derive(Debug)]
pub struct OarsError {
    /// The general category of the error
    error_type: ErrorKind,

    /// A user-friendly description of the array which can supply additional information about
    /// the error.
    desc: String,
}

/// A generic type for anything that can return `OAConstructionError`.
///
/// This type is meant for anything that isn't an orthogonal array constructor.
pub type OarsResult<T> = Result<T, OarsError>;

impl Error for OarsError {
    fn description(&self) -> &str {
        &self.desc
    }
}

impl fmt::Display for OarsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OA Construction Error: {}", &self.desc)
    }
}

impl OarsError {
    pub fn new<T>(kind: ErrorKind, msg: T) -> Self
    where
        T: Into<String>,
    {
        OarsError {
            error_type: kind,
            desc: msg.into(),
        }
    }
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
