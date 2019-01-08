//! The generic interface to define a strong orthogonal array (SOA) and a trait for constructing SOAs
//! This module also defines a few construction methods, as well as provide a verification method to
//! ensure that the resulting points are stratified as an SOA should be.

use ndarray::Array2;

/// The general categories of errors for `SOAConstructionError`
#[derive(Debug)]
pub enum SOACErrorKind {
    /// Invalid parameters were supplied to the constructor
    InvalidParams,

    /// There was a runtime error that prevented the strong orthogonal array from being properly
    /// constructed
    RuntimeError,
}

/// An error indicating that there was some error constructing the strong orthogonal array.
#[derive(Debug)]
pub struct SOAConstructionError {
    /// The general category of the error
    error_type: SOACErrorKind,

    /// A user-friendly description of the array which can supply additional information about
    /// the error.
    desc: String,
}

/// A result type for strong orthogonal array construction
pub type SOAResult = Result<SOA, SOAConstructionError>;

/// A trait that demarcates SOA constructors
pub trait SOAConstructor {
    /// The method that generates an SOA. Any verification for the parameters must be handled by
    /// the constructor itself, and there are no generic interfaces for doing so.
    fn gen(&self) -> SOAResult;
}

/// A structure representing a strong orthogonal array, consisting of the array and associated
/// metadata.
#[derive(Debug)]
pub struct SOA {
    pub strength: u32,

    /// The number of columns in the strong orthogonal array. This is the dimensionality of the point
    /// set.
    pub factors: u32,

    /// The number of times each t-tuple is present in the strong orthogonal array. Setting this to 1
    /// ensures the Latin hypercube guarantee.
    pub index: u32,

    /// The internal array that holds the data for the strong orthogonal array. This is not the same as
    /// the point set that can be used for Monte Carlo simulations.
    pub points: Array2<u32>,
}
