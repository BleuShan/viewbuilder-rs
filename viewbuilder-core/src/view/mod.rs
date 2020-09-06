//! The core view abstractions.

mod impls;

use crate::prelude::*;

/// The core trait used to describe the building blocks of a user interface.
pub trait View
where
    Self: SendSync,
{
    /// The representing the body of the view.
    type Body: View;

    /// The content and behaviour of the view.
    fn body(&self) -> Self::Body;
}
