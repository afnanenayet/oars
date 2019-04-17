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
//! There are also general methods available to verify that a point set matches up with the
//! orthogonal array parameters, and convert points from an orthogonal array to a point set
//! and back.
//!
//! Many of the techniques used in this library were either taken from or inspired by Art Owen's
//! currently unpublished book about Monte Carlo integration. This library was also developed by a
//! member of Dartmouth's Visual Computing Lab under the tutelage of Dr. Wojciech Jarosz.
//!
//! # Example Usage
//!
//! ```
//! use oars::constructors::Bose;
//! use oars::oa::{OAConstructor, normalize};
//!
//! // Configure the parameters for the bose construction
//! let bose = Bose {
//!     prime_base: 3,
//!     dimensions: 3,
//! };
//!
//! // Use the OAConstructor method to generate the orthogonal array
//! let oa = bose.gen().unwrap();
//!
//! // Convert the orthogonal array into a point set usable for Monte Carlo, without jittering
//! let points = normalize(&oa, 0.0, true);
//! ```

pub mod constructors;
pub mod oa;
mod perm_vec;
pub mod soa;
mod utils;
