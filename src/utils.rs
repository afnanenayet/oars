/// Misc utilities and convenience functions for the crate
use std::vec::Vec;

/// Given some number in base 10, convert it to a set of digits that show the
/// number's representation in the provided base.
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
        num = num / base;
    }
    new_base
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
