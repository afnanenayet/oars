/// Misc utilities and convenience functions for the crate
use std::vec::Vec;

/// Convert a number to an arbitrary base representation
///
/// This function returns a vector with the digits of the number. For example,
/// given the number 5 and base 2, the resulting vector will look like:
/// `[1, 0, 1]`.
///
/// Note that this method returns a vector that looks "backwards", that is to
/// say, that the vector is read in the reverse order that you would read a string.
/// This way, the index of the vector gives you the power of the base, and the element
/// at some index would be the coefficient to $b^i$. For a human readable format,
/// reverse the vector.
pub fn to_base(num: u32, base: u32) -> Vec<u32> {
    let mut new_base = Vec::new();
    let mut num = num;

    while num > 0 {
        new_base.push(num % base);
        num /= base;
    }
    new_base
}

/// Convert a number to an arbitrary base with a fixed number of digits
///
/// Given some number, convert the number to some base with a specified number of digits. This
/// means that numbers can be truncated if `degree` is too small. This also means that numbers may
/// be zero-padded.
pub fn to_base_fixed(num: u32, base: u32, degree: u32) -> Vec<u32> {
    // The number in a the new base
    let mut new_base = vec![0; degree as usize];
    let mut new_num = num;

    for i in 0..(degree + 1) {
        let i = i as usize;
        new_base[i] = new_num % base;
        new_num /= base;
    }
    new_base
}

/// Evaluate a polynomial at `position` using the given coefficient vector
///
/// Using a coefficient vector, where the index of the vector signifies the location, evaluate
/// the given polynomial. This method uses Horner's rule to evaluate the polynomial efficiently.
pub fn poly_eval(coeffs: &Vec<u32>, position: u32) -> u32 {
    let mut result = 0;

    for i in (0..coeffs.len()).rev() {
        result = (result * position) + coeffs[i as usize];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base() {
        let num = 5;
        let base = 2;
        let mut res = to_base(num, base);
        res.reverse();
        assert!(res == vec![1, 0, 1]);

        let num = 1;
        let base = 2;
        let mut res = to_base(num, base);
        res.reverse();
        assert!(res == vec![1]);

        let num = 9;
        let base = 2;
        let mut res = to_base(num, base);
        res.reverse();
        assert!(res == vec![1, 0, 0, 1]);

        let num = 7;
        let base = 3;
        let mut res = to_base(num, base);
        res.reverse();
        assert!(res == vec![2, 1]);
    }
}
