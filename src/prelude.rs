//! Exports the necessary traits, structs, and error types to work with oars.
//!
//! Note that this module does not export structs for specific constructors.

pub use crate::utils::{Float, Integer};
pub use crate::oa::{OAConstructor, OA, OAResult, OAConstructionError};
pub use crate::soa::{SOA, SOAConstructor, SOAResult, SOAConstructionError};

#[cfg(feature = "parallel")]
pub use create::oa::{ParOAConstructor};
