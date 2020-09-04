//! The viewbuilder-core prelude
//!
//! Provides commonly used type aliases and trait imports.

#[allow(unused_imports)]
pub(crate) use derive_more::{
    AsMut,
    AsRef,
    Deref,
    DerefMut,
    Display,
    From,
    FromStr,
    Index,
    IndexMut,
    Into,
    IntoIterator,
    TryInto,
};
#[allow(unused_imports)]
pub(crate) use std::{
    convert::{
        AsMut,
        AsRef,
        TryFrom,
    },
    fmt::{
        self,
        Debug,
        Display,
    },
    ops::{
        Deref,
        DerefMut,
        Index,
        IndexMut,
    },
    str::FromStr,
};

/// An alias for the () type. Used to get a more uniform syntax.
pub type Unit = ();
