//! The core view abstractions.

use crate::prelude::*;

/// The core trait used to describe the building blocks of a user interface.
pub trait View
where
    Self: Send + Sync,
{
    /// The representing the body of the view.
    type Body: View;

    /// The content and behaviour of the view.
    fn body(&self) -> Self::Body;
}

impl View for Unit {
    type Body = Self;
    fn body(&self) -> Self::Body {}
}
