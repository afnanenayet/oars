//! Implementation of Galois field arithmetic in the domain 2^m, enabling orthogonal arrays with
//! bases of arbitrary powers of 2.

use crate::utils::{poly_eval, to_base_fixed, Integer};
use num::ToPrimitive;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

/// An integer polynomial. The degree is the size of the vector.
struct Polynomial<'a, T: Integer> {
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

                // can't use adassign because the `num` doesn't implement it
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

/// A finite field with a characteristic irreducible polynomial in some domain that is a power of
/// 2.
struct Field<T: Integer> {
    /// The coefficients for a primitive polynomial that is a characteristic polynomial for a field
    /// of `size` in base 10.
    prim_poly: T,

    /// The size of the domain of the finite field (must be a power of 2)
    size: T,
}
