//! Experimental constructors for strong orthogonal arrays

use crate::utils::Integer;
use crate::{
    oa::OA,
    soa::{SOAConstructor, SOAResult},
};
use ndarray::{prelude::*, Array2, Axis};
use oars_proc_macro::Checked;

/// The original SOA construction, as described by He and Tang
///
/// He and Tang described how to construct a strong orthogonal array from a regular orthogonal
/// array. This involves taking a semi-embeddable orthogonal array, converting it to a generalized
/// orthogonal array, and then applying a transformation to collapse the column groups of the GOA
/// and an optional permutation in order to create the final SOA.
#[derive(Checked)]
pub struct HeTang {}

impl SOAConstructor for HeTang {
    fn gen(&self) -> SOAResult {
        // TODO
        unimplemented!();
    }
}

/// Create a generalized orthogonal array from a semi-embeddable orthogonal array
///
/// Creating a generalized orthogonal array (GOA) is an intermediate step used for SOA
/// construction. The process for creating GOAs as well as embeddability for orthogonal arrays is
/// defined in He and Tang's [paper](https://arxiv.org/pdf/1407.0204.pdf).
fn oa_to_goa<T: Integer>(oa: &OA<T>) -> Array2<T> {
    // TODO right now this is special cased for OAs of strength 3
    if oa.strength != T::from(3).unwrap() {
        panic!("This method has not been implemented for any orthogonal arrays of strength != 3");
    }

    // We create a new array with `strength` * `m'` factors (multiply the dimensions by the strength
    // of the original OA). The new array will have the same number of rows as the original OA.
    let m_prime = oa.factors.to_usize().unwrap() - 1;
    let goa_factors = oa.strength.to_usize().unwrap() * m_prime;
    let n = oa.points.len_of(Axis(1));
    let mut goa = ndarray::Array2::zeros((goa_factors, n));

    // We use equations 3.1-3.3 in the Vicky thesis to map columns to the GOA
    // TODO use more elegant syntax to copy over a whole column

    // Equation 3.1
    // Map every first element in the column groups in the GOA to the columns of the OA
    // $(b_{11}, ... b_{m'1} = (a_1, ..., a_{m'}) $
    for oa_col_idx in 0..m_prime {
        // need to map the oa_col_idx to the index it corresponds to in the GOA
        let goa_col_idx = oa_col_idx * oa.strength.to_usize().unwrap();

        for i in 0..n {
            goa[[goa_col_idx, i]] = oa[[oa_col_idx, i]];
        }
    }

    // Equation 3.2
    // $(b_{12}, ... b_{m'2} = (a_m, ..., a_m) $
    for oa_col_idx in 0..m_prime {
        // need to map the oa_col_idx to the index it corresponds to in the GOA
        let goa_col_idx = (oa_col_idx * oa.strength.to_usize().unwrap()) + 1;

        for i in 0..n {
            goa[[goa_col_idx, i]] = oa[[oa.factors.to_usize().unwrap(), i]];
        }
    }

    // Equation 3.3
    // $(b_{13}, ... b_{m'3} = (a_2, ..., a_{m'}, a_1) $
    for oa_col_idx in 0..m_prime {
        let goa_col_idx = (oa_col_idx * oa.strength.to_usize().unwrap()) + 2;
        let oa_col = (oa_col_idx + 1) % m_prime;

        for i in 0..n {
            goa[[goa_col_idx, i]] = oa[[oa_col, i]];
        }
    }
    goa
}

/// Reduce a generalized orthogonal array to a strong orthogonal array
///
/// Given a generalized orthogonal array (GOA), and its properties, perform the transformation as
/// described by He and Tang and return the resultant strong orthogonal array.
fn goa_to_soa<T: Integer>(arr: &Array2<T>, s: T, m_prime: usize) -> Array2<T> {
    unimplemented!();
}

#[cfg(test)]
mod tests {}
