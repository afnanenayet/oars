//! The generic interface to define a strong orthogonal array (SOA) and a trait for constructing SOAs
//! This module also defines a few construction methods, as well as provide a verification method to
//! ensure that the resulting points are stratified as an SOA should be.

use ndarray::Array2;
use serde_derive::{Deserialize, Serialize};

/// The general categories of errors for `SOAConstructionError`
#[derive(Debug, Serialize, Deserialize)]
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
    /// The strength of the strong orthogonal array
    pub strength: u32,

    /// The base `s` used to derive the number of levels and total number of samples
    pub base: u32,

    /// The internal array that holds the data for the strong orthogonal array
    pub points: Array2<u32>,
}

/// Verify whether a point set is a valid strong orthogonal array based on the metadata supplied in
/// that struct. This method returns whether the given SOA is valid, based on the metadata. It will
/// check that the SOA maintains the stratification guarantees based on the properties of the SOA.
pub fn verify_soa(soa: &SOA) -> bool {
    // TODO(afnan)
    // - Find some way to find every combo of numbers that adds up to `t`
    // - Write some method that generates the unshuffled stratification guarantees
    // - Check that each strata are equally filled
    // - Write unit tests
    false
}
