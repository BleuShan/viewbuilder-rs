//! The viewbuilder-core prelude
//!
//! Provides commonly used type aliases and trait imports.

/// An alias for the () type. Used to get a more uniform syntax.
pub type Unit = ();

/// A stand-in for the ! type.
pub type Never = core::convert::Infallible;
