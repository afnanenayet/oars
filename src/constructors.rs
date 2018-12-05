//! Various constructors that implement the `OAConstructor` trait, and provide
//! methods to generate orthogonal arrays. Each struct contains the
//! configuration parameters necessary for the construction method, and the
//! `gen` method (from the trait) will construct an orthogonal array from the
//! given parameters.
//!
//! The description of the construction method will be with the struct that contains
//! the parameters.

use crate::oa::{OAConstructor, OA};

/// The Bush construction technique, as described by Art Owen in his currently unpublished Monte
/// Carlo textbook. In Chapter 10.4, he describes the Bush construction technique. It's only
/// parameter is the prime number base.
struct Bush {
    /// The strength of the orthogonal array. It *must* be a prime number.
    prime_base: u32,
}

impl Bush {
    /// Verify the parameters for Bush construction. This checks to see whether the prime base
    /// is valid and returns whether the parameters are correct.
    fn verify_params(&self) -> bool {
        true
    }
}

impl OAConstructor for Bush {
    fn gen(&self) -> OA {
    }
}
