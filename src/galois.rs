//! Implementation of Galois field arithmetic in the domain 2^m, enabling orthogonal arrays with
//! bases of arbitrary powers of 2.

use crate::utils::{poly_eval, to_base_fixed, Integer};
use num::{ToPrimitive, pow};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use primes::is_prime;

/// An integer polynomial. The degree is the size of the vector.
pub struct Polynomial<'a, T: Integer> {
    /// The vector representing the polynomial
    poly: Vec<T>,

    /// The field that this polynomial is a member of
    field: &'a Field<T>,
}

impl<'a, T: Integer> Polynomial<'a, T> {
    /// Create a new `Polynomial` from some integer with a given degree.
    pub fn new(num: T, degree: T, field: &'a Field<T>) -> Self {
        Self {
            poly: to_base_fixed(num, T::from(2).unwrap(), degree),
            field,
        }
    }

    /// The degree of the polynomial
    ///
    /// Returns the degree of the polynomial.
    pub fn degree(&self) -> usize {
        self.poly.len()
    }
}

impl<'a, T: Integer> Add for Polynomial<'a, T> {
    type Output = Self;

    /// Add a member of a finite field with another member of the same finite field. The vectors
    /// must be of the same length, otherwise this method will panic.
    fn add(self, other: Self) -> Self {
        if other.poly.len() != self.poly.len() {
            panic!("Both polynomials must have the same degree");
        }
        let mut res = vec![T::from(0).unwrap(); self.poly.len().to_usize().unwrap()];

        // Just xor each element
        for (idx, val) in self.poly.iter().enumerate() {
            let temp_res = (*val).to_u64().unwrap() ^ other.poly[idx].to_u64().unwrap();
            res[idx] = T::from(temp_res).unwrap();
        }
        Self {
            poly: res,
            field: self.field,
        }
    }
}

impl<'a, T: Integer> AddAssign for Polynomial<'a, T> {
    /// AddAssign a member of a finite field to another member of the same finite field. The
    /// vectors must be of the same length, otherwise this method will panic.
    fn add_assign(&mut self, other: Self) {
        if other.poly.len() != self.poly.len() {
            panic!("Both polynomials must have the same degree");
        }
        // Just xor each element in-place
        for (idx, val) in self.poly.iter_mut().enumerate() {
            let temp_res = (*val).to_u64().unwrap() ^ other.poly[idx].to_u64().unwrap();
            *val = T::from(temp_res).unwrap();
        }
    }
}

impl<'a, T: Integer> Sub for Polynomial<'a, T> {
    type Output = Self;

    /// Subtract a member of a finite field with another member of the same finite field. The
    /// vectors must be of the same length, otherwise this method will panic.
    fn sub(self, other: Self) -> Self {
        if other.poly.len() != self.poly.len() {
            panic!("Both polynomials must have the same degree");
        }
        let mut res = vec![T::from(0).unwrap(); self.poly.len().to_usize().unwrap()];

        // Just xor each element
        for (idx, val) in self.poly.iter().enumerate() {
            let temp_res = (*val).to_u64().unwrap() ^ other.poly[idx].to_u64().unwrap();
            res[idx] = T::from(temp_res).unwrap();
        }
        Self {
            poly: res,
            field: self.field,
        }
    }
}

impl<'a, T: Integer> SubAssign for Polynomial<'a, T> {
    /// SubAssign a member of a finite field to another member of the same finite field. The
    /// vectors must be of the same length, otherwise this method will panic.
    fn sub_assign(&mut self, other: Self) {
        if other.poly.len() != self.poly.len() {
            panic!("Both polynomials must have the same degree");
        }
        // Just xor each element
        for (idx, val) in self.poly.iter_mut().enumerate() {
            let temp_res = (*val).to_u64().unwrap() ^ other.poly[idx].to_u64().unwrap();
            *val = T::from(temp_res).unwrap();
        }
    }
}

impl<'a, T: Integer> Mul for Polynomial<'a, T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // The resulting degree of the polynomial
        let res_degree = self.degree() + rhs.degree() - 1;
        let mut prod = vec![T::from(0).unwrap(); res_degree];

        for (idx_a, val_a) in self.poly.iter().enumerate() {
            for (idx_b, val_b) in rhs.poly.iter().enumerate() {
                let new_idx = idx_a + idx_b;

                // can't use add assign because the `num` doesn't implement it
                prod[new_idx] = prod[new_idx] + (*val_a * *val_b);
            }
        }

        // The coefficients from the product multiplication interpreted in base 10
        let prod_b_10 = poly_eval(&prod, T::from(2).unwrap());
        let result = prod_b_10.to_u64().unwrap() % self.field.prim_poly.to_u64().unwrap();
        let result_coeffs = to_base_fixed(
            T::from(result).unwrap(),
            T::from(2).unwrap(),
            T::from(self.degree()).unwrap(),
        );

        Self {
            poly: result_coeffs,
            field: self.field,
        }
    }
}

/// Calculate the characteristic polynomial for a field of size 2^power.
///
/// This method calculates the "primitive polynomial" for GF(2^power), returning the coefficients
/// of the polynomial for base 2 interpreted in base 10.
///
/// If no such polynomial can be found, this method will return `None`.
fn prim_poly<T: Integer>(power: T) -> Option<T> {
    // The polynomial must be of degree `power`
    let power_raised = pow(2, power.to_usize().unwrap());
    //let upper_bound = power_raised | (power_raised - 1);

    // Figure out some combination of exponents that adds up to a prime number, that includes
    // `power`
    for i in (1..power_raised).rev() {
        let candidate = power_raised + i;
        let mut temp_candidate = candidate;
        let mut power_sum = 0;
        let mut idx = 0;

        while temp_candidate > 0 {
            power_sum = (temp_candidate % 2) * idx;
            idx += 1;
            temp_candidate /= 2;
        }

        if is_prime(power_sum.to_u64().unwrap()) {
            // The reason we add 1 is because the algorithm doesn't factor for the 0 power, which
            // is 1. We shift the number to the right by multipling by 2, and add 1. This preserves
            // the sum of the powers, while giving us the proper polynomial.
            return Some(T::from(candidate).unwrap());
        }
    }
    None
}

/// A finite field with a characteristic irreducible polynomial in some domain that is a power of
/// 2.
pub struct Field<T: Integer> {
    /// The coefficients for a primitive polynomial that is a characteristic polynomial for a field
    /// of `size` in base 10.
    prim_poly: T,

    /// The size of the domain of the finite field (must be a power of 2)
    size: T,
}

impl<T: Integer> Field<T> {
    /// Initialize a field of a particular size
    ///
    /// Given the size of a finite field, this method will calculate the primitive polynomial for
    /// that field. The size must be a power of 2.
    pub fn new(size: T) -> Self {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prim_poly() {
        // Test cases taken from:
        // http://mathworld.wolfram.com/PrimitivePolynomial.html
        let p = prim_poly(2);
        assert!(p.unwrap() == 7);

        let p = prim_poly(3);
        assert!(p.unwrap() == 15);

        let p = prim_poly(4);
        assert!(p.unwrap() == 17);
    }
}
