//! Experimental constructors for strong orthogonal arrays

use crate::{
    oa::OA,
    soa::SOA,
    soa::{SOAConstructor, SOAResult},
    utils::Integer,
};
use ndarray::{prelude::*, Array2, Axis};
use num::pow;
use oars_proc_macro::Checked;

/// The original SOA construction, as described by He and Tang
///
/// He and Tang described how to construct a strong orthogonal array from a regular orthogonal
/// array. This involves taking a semi-embeddable orthogonal array, converting it to a generalized
/// orthogonal array, and then applying a transformation to collapse the column groups of the GOA
/// and an optional permutation in order to create the final SOA.
#[derive(Checked)]
pub struct HeTang<'a, T: Integer> {
    /// The OA to generate the SOA from
    pub oa: &'a OA<T>,
}

impl<'a, T: Integer> SOAConstructor for HeTang<'a, T> {
    fn gen(&self) -> SOAResult {
        let m_prime = self.oa.factors.to_usize().unwrap() - 1;
        let goa = oa_to_goa(&self.oa);
        let soa = goa_to_soa(&goa, self.oa.strength, self.oa.levels, m_prime);
        Ok(SOA {
            strength: self.oa.strength.to_u32().unwrap(),
            base: self.oa.levels.to_u32().unwrap(),
            points: soa,
        })
    }
}

/// Create a generalized orthogonal array from a semi-embeddable orthogonal array
///
/// Creating a generalized orthogonal array (GOA) is an intermediate step used for SOA
/// construction. The process for creating GOAs as well as embeddability for orthogonal arrays is
/// defined in He and Tang's [paper](https://arxiv.org/pdf/1407.0204.pdf).
fn oa_to_goa<T: Integer>(oa: &OA<T>) -> Array2<T> {
    // TODO generalize this to more strengths
    if oa.strength != T::from(3).unwrap() {
        panic!("This method has not been implemented for any orthogonal arrays of strength != 3");
    }

    // We create a new array with `strength` * `m'` factors (multiply the dimensions by the strength
    // of the original OA). The new array will have the same number of rows as the original OA.
    // In Liu, Liu p 1724, they give us the formula to generically find m prime
    // m' = \floor{2(m - 1) / (t - 1)}
    // TODO(afnan) implement the generic m' formula instead of hardcoding the one for t = 3
    //let m_prime = oa.factors.to_usize().unwrap() - 1;
    let m = oa.factors.to_usize().unwrap() - 1;
    let t = oa.strength.to_usize().unwrap();
    let m_prime = 2 * (m - 1) / (t - 1);
    let goa_factors = oa.strength.to_usize().unwrap() * m_prime;
    let n = oa.points.len_of(Axis(0));
    let mut goa = ndarray::Array2::zeros((n, goa_factors));

    // We use equations 3.1-3.3 in the Vicky thesis to map columns to the GOA
    // TODO use more elegant syntax to copy over a whole column

    // Equation 3.1
    // Map every first element in the column groups in the GOA to the columns of the OA
    // $(b_{11}, ... b_{m'1} = (a_1, ..., a_{m'}) $
    for oa_col_idx in 0..m_prime {
        // need to map the oa_col_idx to the index it corresponds to in the GOA
        let goa_col_idx = oa_col_idx * oa.strength.to_usize().unwrap();

        for i in 0..n {
            goa[[i, goa_col_idx]] = oa[[i, oa_col_idx]];
        }
    }

    // Equation 3.2
    // $(b_{12}, ... b_{m'2} = (a_m, ..., a_m) $
    for oa_col_idx in 0..m_prime {
        // need to map the oa_col_idx to the index it corresponds to in the GOA
        let goa_col_idx = (oa_col_idx * oa.strength.to_usize().unwrap()) + 1;

        for i in 0..n {
            goa[[i, goa_col_idx]] = oa[[i, oa.factors.to_usize().unwrap() - 1]];
        }
    }

    // Equation 3.3
    // $(b_{13}, ... b_{m'3} = (a_2, ..., a_{m'}, a_1) $
    for oa_col_idx in 0..m_prime {
        let goa_col_idx = (oa_col_idx * oa.strength.to_usize().unwrap()) + 2;
        let oa_col = (oa_col_idx + 1) % m_prime;

        for i in 0..n {
            goa[[i, goa_col_idx]] = oa[[i, oa_col]];
        }
    }
    goa
}

/// Reduce a generalized orthogonal array to a strong orthogonal array
///
/// Given a generalized orthogonal array (GOA), and its properties, perform the transformation as
/// described by He and Tang and return the resultant strong orthogonal array.
fn goa_to_soa<T: Integer>(arr: &Array2<T>, strength: T, levels: T, m_prime: usize) -> Array2<u32> {
    let strength = strength.to_usize().unwrap();
    let n = arr.len_of(Axis(0));
    let mut soa = Array2::<u32>::zeros((n, m_prime));

    // Reduce the GOA to an SOA
    for col in 0..m_prime {
        for i in 0..n {
            let mut res = 0;

            for offset in 0..strength {
                let goa_col = (col * 3) + offset;

                // the number to multiply the coefficient by (the coefficient being the number in
                // the GOA)
                let power = pow(levels, strength - offset - 1);
                res = res + (power * arr[[i, goa_col]]).to_u32().unwrap();
            }
            soa[[i, col]] = res;
        }
    }
    soa
}

/// Perform random digit scrambling on a strong orthogonal array
///
/// This method of scrambling performs a form of shuffling on the strong orthogonal array that
/// should be robust and maintain the stratification properties of the original SOA. This method
/// will create a new SOA and will not modify the original struct.
fn random_digit_scramble(soa: &SOA) -> SOA {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constructors::Bush, oa::OAConstructor, soa::verify};
    use ndarray::array;

    #[test]
    fn test_soa_pipeline() {
        // Example taken from Vicky MSc thesis, figures (3.5 - 3.7)
        let oa_pts = array![
            [0, 0, 0, 0],
            [0, 0, 1, 1],
            [0, 1, 0, 1],
            [0, 1, 1, 0],
            [1, 0, 0, 1],
            [1, 0, 1, 0],
            [1, 1, 0, 0],
            [1, 1, 1, 1],
        ];
        let oa = OA {
            factors: 4,
            strength: 3,
            levels: 2,
            index: 1,
            points: oa_pts,
        };

        // The expected SOA (Vikcy, Fig. 3.6)
        let ground_truth = array![
            [0, 0, 0],
            [2, 3, 6],
            [3, 6, 2],
            [1, 5, 4],
            [6, 2, 3],
            [4, 1, 5],
            [5, 4, 1],
            [7, 7, 7],
        ];
        let goa = oa_to_goa(&oa);
        let soa = goa_to_soa(&goa, oa.strength, oa.levels, 3);
        assert!(soa == ground_truth);
    }

    #[test]
    fn test_he_tang_base_7_str_3_dim_5() {
        let bush = Bush {
            prime_base: 7,
            strength: 3,
            dimensions: 5,
        };
        let oa = bush.gen().unwrap();
        let ht = HeTang { oa: &oa };
        let soa = ht.gen().unwrap();
        assert!(verify(&soa));
    }

    #[test]
    fn test_he_tang_base_7_str_3_dim_4() {
        let bush = Bush {
            prime_base: 7,
            strength: 3,
            dimensions: 4,
        };
        let oa = bush.gen().unwrap();
        let ht = HeTang { oa: &oa };
        let soa = ht.gen().unwrap();
        assert!(verify(&soa));
    }

    #[test]
    fn test_he_tang_base_17_str_3_dim_6() {
        let bush = Bush {
            prime_base: 17,
            strength: 3,
            dimensions: 6,
        };
        let oa = bush.gen().unwrap();
        let ht = HeTang { oa: &oa };
        let soa = ht.gen().unwrap();
        assert!(verify(&soa));
    }

    #[test]
    fn test_he_tang_base_17_str_3_dim_11() {
        let bush = Bush {
            prime_base: 17,
            strength: 3,
            dimensions: 11,
        };
        let oa = bush.gen().unwrap();
        let ht = HeTang { oa: &oa };
        let soa = ht.gen().unwrap();
        assert!(verify(&soa));
    }

    #[test]
    fn test_he_tang_base_17_str_3_dim_5() {
        let bush = Bush {
            prime_base: 17,
            strength: 3,
            dimensions: 5,
        };
        let oa = bush.gen().unwrap();
        let ht = HeTang { oa: &oa };
        let soa = ht.gen().unwrap();
        assert!(verify(&soa));
    }

    #[test]
    fn test_he_tang_base_7_str_3_dim_3() {
        let bush = Bush {
            prime_base: 7,
            strength: 3,
            dimensions: 3,
        };
        let oa = bush.gen().unwrap();
        let ht = HeTang { oa: &oa };
        let soa = ht.gen().unwrap();
        assert!(verify(&soa));
    }
}
