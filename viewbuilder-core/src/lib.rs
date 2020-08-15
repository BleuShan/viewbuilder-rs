//! Core viewbuilder abstractions.

#![forbid(future_incompatible)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]

pub mod layout;
pub mod prelude;
pub mod view;

pub use view::View;
