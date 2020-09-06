//! Core viewbuilder abstractions.

#![forbid(future_incompatible)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(never_type, format_args_capture)]
#![doc(test(
    no_crate_inject,
    attr(forbid(future_incompatible)),
    attr(warn(nonstandard_style, rust_2018_idioms)),
    attr(feature(never_type, format_args_capture))
))]

pub mod prelude;
pub mod transform;
pub mod view;

pub use view::View;
