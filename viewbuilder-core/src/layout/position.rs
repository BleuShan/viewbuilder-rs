//! Position

use crate::prelude::*;
use num_traits::Zero;
use simba::simd::{
    f64x2,
    SimdValue,
};

/// Origin Lane indicies
const X: usize = 0;
const Y: usize = 1;

/// A position in 2D space
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    origin: f64x2,
}

impl Position {
    /// Creates a position at the given coordinate
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self::from(f64x2::new(x, y))
    }

    /// Creates a position at origin point
    #[inline]
    pub fn origin() -> Self {
        let origin = f64x2::zero();
        Self::from(origin)
    }

    /// Return the x component
    #[inline]
    pub fn x(&self) -> f64 {
        self.origin.extract(X)
    }

    /// Return the y component
    #[inline]
    pub fn y(&self) -> f64 {
        self.origin.extract(Y)
    }
}

impl From<f64x2> for Position {
    #[inline]
    fn from(origin: f64x2) -> Self {
        Self { origin }
    }
}

impl Default for Position {
    #[inline]
    fn default() -> Self {
        Self::origin()
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_has_all_zero_coordinates() {
        let origin = Position::origin();

        assert_eq!(origin.x(), origin.y());
        assert_eq!(origin.x(), 0.0)
    }

    #[test]
    fn new_should_set_the_correct_coordinates() {
        let position = Position::new(1.0, 2.0);

        assert_eq!(position.x(), 1.0);
        assert_eq!(position.y(), 2.0)
    }

    #[test]
    fn debug_should_show_coord() {
        let x = 1.0;
        let y = 2.0;
        let position = Position::new(x, y);
        let type_name = std::any::type_name::<Position>();

        assert_eq!(
            format!("{position:?}"),
            format!("{type_name} {{ x: {x:?}, y: {y:?} }}")
        )
    }
}
