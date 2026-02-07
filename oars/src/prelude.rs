//! Exports the necessary traits, structs, and error types to work with oars.
//!
//! Note that this module does not export structs for specific constructors.

#[cfg(feature = "parallel")]
pub use crate::oa::ParOAConstructor;
pub use crate::oa::{OA, OAConstructor, OAResult};
pub use crate::soa::{SOA, SOAConstructor, SOAResult};
pub use crate::utils::{Float, Integer, OarsError, OarsResult};
