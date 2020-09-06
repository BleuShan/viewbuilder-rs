//! [`view`](crate::view) module default implementations.

use super::*;

impl View for Unit {
    type Body = Self;
    fn body(&self) -> Self::Body {}
}
