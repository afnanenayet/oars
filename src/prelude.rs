//! Exports the necessary traits, structs, and error types to work with oars.
//!
//! Note that this module does not export structs for specific constructors.

#[cfg(feature = "parallel")]
pub use crate::oa::ParOAConstructor;
pub use crate::oa::{OAConstructor, OAResult, OA};
pub use crate::soa::{SOAConstructionError, SOAConstructor, SOAResult, SOA};
pub use crate::utils::{Float, Integer, OarsError, OarsResult};
