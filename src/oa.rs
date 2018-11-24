//! The generic interface to define an orthogonal array and generic construction methods. This
//! module also defines a few construction methods.

use ndarray::ArrayD;

/// The definition of an orthogonal array with its point set and parameters.
struct OA {
    /// The size of the set $X$ that the array can select elements from.
    levels: u32,

    /// The size of the t-tuple. In other words, this is the dimensionality of the stratification
    /// guarantee.
    strength: u32,

    /// The number of columns in the orthogonal array. This is the dimensionality of the point
    /// set.
    factors: u32,

    /// The number of times each t-tuple is present in the orthogonal array. Setting this to 1
    /// ensures the Latin hypercube guarantee.
    index: u32,

    /// The internal array that holds the actual point set
    points: ArrayD<u32>,
}

/// Normalize an orthogonal array into a point set using Art Owen's normalization technique.
fn normalize(oa: &OA) -> ArrayD<f32> {}

/// A generic trait to demarcate orthogonal array constructors
trait OAConstructor {
    /// The method that generates an orthogonal array. Any necessary parameters must be handled
    /// by the constructor itself.
    fn gen(&self) -> OA;
}
