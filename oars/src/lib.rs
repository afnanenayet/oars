//! # Summary
//!
//! The oars crate is a Rust library designed to faciliate the construction, verification, and
//! usage of orthogonal arrays, as well as strong orthogonal arrays for statistical experiments
//! and Monte Carlo simulations.
//!
//! There is an `OAConstructor` trait which marks whether a struct has a method that can construct
//! an orthogonal array. The structs generally take the parameters necessary for configuring the
//! construction method.
//!
//! There is also a `ParOAConstructor` trait which demarcates whether a struct has a method that
//! can construct an orthogonal array utilizing parallelization. In order to use this trait, you
//! must enable the `parallel` feature for the crate.
//!
//! There are also general methods available to verify that a point set matches up with the
//! orthogonal array parameters, and convert points from an orthogonal array to a point set
//! and back. These verification utilities are also provided for strong orthogonal arrays, though
//! this crate does not yet provide an efficient constructor for strong orthogonal arrays.
//!
//! Many of the techniques used in this library were either taken from or inspired by Art Owen's
//! currently unpublished [book](https://statweb.stanford.edu/~owen/mc/) about Monte Carlo
//! integration. This library was also developed through the Dartmouth [Visual Computing
//! Lab](http://vcl.cs.dartmouth.edu/) under the tutelage of [Dr. Wojciech
//! Jarosz](https://cs.dartmouth.edu/~wjarosz/).
//!
//! # Example Usage
//!
//! ```
//! use oars::prelude::*;
//! use oars::constructors::{Bose, BoseChecked};
//! use oars::oa::{normalize, verify};
//!
//! # fn main() -> OarsResult<()> {
//! // Configure the parameters for the Bose construction, using the checked variant so we can make
//! // sure that the supplied parameters are valid.
//! let bose = BoseChecked {
//!     prime_base: 3,
//!     dimensions: 3,
//! };
//!
//! // Use the OAConstructor method to generate the orthogonal array
//! let oa = bose.verify()?.gen()?;
//!
//! // Verify that the orthogonal array is correct according to its parameters
//! assert!(verify(&oa)?);
//!
//! // Convert the orthogonal array into a point set usable for Monte Carlo, without jittering
//! let points = normalize(&oa, 0.0, true)?;
//! # Ok(())
//! # }
//! ```

pub mod constructors;
pub mod oa;
mod perm_vec;
pub mod prelude;
pub mod soa;
mod utils;

// Export these types because any consumer of this library will need to have these type definitions
// in order to use the OA/SOA definitions and constructors
pub use utils::{ErrorKind, Float, Integer, OarsError, OarsResult};
