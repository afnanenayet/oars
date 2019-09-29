//! Implementations of different orthogonal array construction techniques.
//!
//! Various constructors that implement the `OAConstructor`/`ParOAConstructor` traits, and provide
//! methods to generate orthogonal arrays. Each struct contains the configuration parameters
//! necessary for the construction method, and the `gen` method (from the trait) will construct an
//! orthogonal array from the given parameters.
//!
//! The description of the construction method is the struct that contains the parameters for that
//! particular method.
//!
//! The constructors offer parameter checked and non-checked variants. The parameter checked
//! variants ensure that the supplied parameters are valid so that the resultant orthogonal array
//! will be well-formed at the expense of some computational overhead. The constructors can be used
//! directly to avoid this overhead, or if the user is certain that the supplied parameters are
//! correct.
//!
//! Parameter checking is implemented using stateful types. The parameters can be supplied to the
//! checked variant of a construction struct, which will be consumed when the verification method
//! is called, yielding a `Result` with the constructor that implements the trait necessary to
//! generate an orthogonal array.
//!
//! For example:
//!
//! ```
//! use oars::prelude::*;
//! use oars::constructors::{Bose, BoseChecked};
//!
//! # fn main() -> OarsResult<()> {
//!
//! // Construct the checked variant
//! let b = BoseChecked {
//!     prime_base: 3,
//!     dimensions: 2,
//! };
//!
//! let oa = b.verify()?.gen();
//! # Ok(())
//! # }
//! ```

// We declare each constructor in their own file to avoid maintaining a massive file of
// constructors
mod bose;
mod bush;
mod liu_liu;
mod soa;

// Re-export child modules so constructors can be used as `constructors::some_constructor`
pub use bose::Bose;
pub use bose::BoseChecked;
pub use bush::Bush;
pub use bush::BushChecked;
pub use liu_liu::LiuLiu;
pub use soa::HeTang;
