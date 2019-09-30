use crate::{
    oa::OA,
    soa::{SOAConstructor, SOAResult, SOA},
    utils::Integer,
};
use ndarray::{s, Array1, Array2};
use num::{pow, ToPrimitive};

/// The construction method introduced by Liu & Liu (2015)
///
/// This is an implementation of the SOA construction method developed by Liu & Liu's 2015 paper.
/// This provides a method of constructing an SOA from an OA and a construction matrix.
pub struct LiuLiu<'a, T: Integer> {
    /// The OA to use to create the SOA
    pub oa: &'a OA<T>,
}

impl<'a, T: Integer> SOAConstructor for LiuLiu<'a, T> {
    fn gen(&self) -> SOAResult {
        let t = self.oa.strength.to_u32().unwrap();
        let m = self.oa.factors.to_u32().unwrap();

        // Calculate `k` and `q` based on `m = kt + q` knowing that `q` must be less than t. We can
        // easily calculate this by doing a rounded integer division (m / t) and let q be the
        // remainder. (Liu & Liu, 1716).
        let q = m % t;

        // There are some special provisions for when q is >= t / 2. If this is true, we will have
        // to add an extra column and an extra row to the $R_1$ matrix.
        let extra_col = q >= (t / 2);
        let r = self.gen_even_r(extra_col);
        let points = self.oa.points.map(|x| x.to_u32().unwrap()) * r;
        Ok(SOA {
            points,
            strength: self.oa.strength.to_u32().unwrap(),
            base: self.oa.levels.to_u32().unwrap(),
        })
    }
}

impl<'a, T: Integer> LiuLiu<'a, T> {
    /// This creates the R matrix for the scenario where an even strength is being used to generate
    /// the SOA. This is a helper method for the main generation method.
    ///
    /// `extra_col` refers to whether `q` is greater than or equal to `t / 2` (as stated by the
    /// paper).
    fn gen_even_r(&self, extra_col: bool) -> Array2<u32> {
        let t = self.oa.strength.to_u32().unwrap();
        let m = self.oa.factors.to_u32().unwrap();
        let s = self.oa.levels.to_u32().unwrap();

        // Calculate `k` and `q` based on `m = kt + q` knowing that `q` must be less than t. We can
        // easily calculate this by doing a rounded integer division (m / t) and let q be the
        // remainder. (Liu & Liu, 1716).
        let k = m / t;
        let q = m % t;

        // Create the V_1 matrix as described in Liu & Liu, p. 1716.
        let v_1 = Array2::from(
            (0..t)
                .map(|i| {
                    let i = i.to_u32().unwrap();
                    [i, t - i - 1]
                })
                .collect::<Vec<[u32; 2]>>(),
        );

        // Liu, Liu describes r_1 as an m by 2k (or + 1) matrix based on q. Since everything is
        // already zeroed out except for the v_1 blocks, we don't have to worry about explicitly
        // setting the zero submatrices.
        let r_1_dims = if extra_col {
            (m.to_usize().unwrap(), 2 * k.to_usize().unwrap())
        } else {
            (m.to_usize().unwrap(), 2 * k.to_usize().unwrap() + 1)
        };
        let mut r_1 = Array2::<u32>::zeros(r_1_dims);

        // Add the V_1 matrices to R_1
        for i in 0..k.to_usize().unwrap() {
            // Determine the indices of where V_1 will be copied into R_1
            // TODO(afnan) check if the ranges are inclusive (if so, we need to add 1 to the
            // *_right indices).
            let top_left = k.to_usize().unwrap() * i;
            let top_right = top_left + 1;
            let bottom_left = (k * t).to_usize().unwrap();
            let bottom_right = bottom_left + 1;
            r_1.slice_mut(s![top_left..top_right, bottom_left..bottom_right])
                .assign(&v_1);
        }

        // create the $d$ vector if necessary and add it to r_1
        if extra_col {
            let mut d = Array1::zeros(m.to_usize().unwrap());

            for i in 0..q.to_usize().unwrap() {
                d[i] = pow(s, i);
            }

            for i in 1..=(q.to_usize().unwrap()) {
                let idx = d.len() - i;
                d[idx] = pow(s, t.to_usize().unwrap() - i);
            }
            r_1.slice_mut(s![.., 2 * k.to_usize().unwrap()]).assign(&d);
        }
        r_1
    }
}

#[cfg(test)]
mod test {
    // TODO(afnan) add tests/sanity checks
}
