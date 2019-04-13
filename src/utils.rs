//! Misc utilities and convenience functions for the crate

use num::{Integer, NumCast, ToPrimitive};
use std::clone::Clone;
use std::vec::Vec;

/// Convert a number to an arbitrary base with a fixed number of digits
///
/// Given some number, convert the number to some base with a specified number of digits. This
/// means that numbers can be truncated if `degree` is too small. This also means that numbers may
/// be zero-padded.
pub fn to_base_fixed<T>(num: T, base: T, degree: T) -> Vec<T>
where
    T: Integer + NumCast + Clone + Copy,
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

/// Evaluate a polynomial at `position` using the given coefficient vector
///
/// Using a coefficient vector, where the index of the vector signifies the location, evaluate
/// the given polynomial. This method uses Horner's rule to evaluate the polynomial efficiently.
pub fn poly_eval<T>(coeffs: &[T], position: T) -> T
where
    T: Integer + NumCast + Copy,
{
    let mut result: T = T::from(0).unwrap();

    // Using Horner's rule
    for i in (0..coeffs.len()).rev() {
        result = (result * position) + coeffs[i.to_usize().unwrap()];
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
}
