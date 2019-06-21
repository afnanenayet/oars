//! Exports the necessary traits, structs, and error types to work with oars.
//!
//! Note that this module does not export structs for specific constructors.

pub use crate::oa::{OAConstructionError, OAConstructor, OAResult, OA};
pub use crate::soa::{SOAConstructionError, SOAConstructor, SOAResult, SOA};
pub use crate::utils::{Float, Integer};

#[cfg(feature = "parallel")]
pub use create::oa::ParOAConstructor;
