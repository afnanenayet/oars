//! Implementations of different orthogonal array construction techniques
//!
//! Various constructors that implement the `OAConstructor`/`ParOAConstructor` traits, and provide
//! methods to generate orthogonal arrays. Each struct contains the configuration parameters
//! necessary for the construction method, and the `gen` method (from the trait) will construct an
//! orthogonal array from the given parameters.
//!
//! The description of the construction method is the struct that contains the parameters for that
//! particular method.

// We declare each constructor in their own file to avoid maintaining a massive file of
// constructors
mod bose;
mod bush;

// Re-export child modules so constructors can be used as `constructors::some_constructor`
pub use bose::Bose;
pub use bush::Bush;
