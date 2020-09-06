#![forbid(future_incompatible)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(
    never_type,
    format_args_capture,
    unboxed_closures,
    fn_traits,
    trait_alias,
    external_doc
)]
#![doc(include = "../../README.md")]

pub mod prelude;
pub use viewbuilder_core::{
    transform,
    View,
};
