//! Transform operations traits.

mod impls;

use crate::prelude::*;

/// A transform operation.
///
/// Objects implementing this trait should represent a transform operation over an arbitrary domain.
/// An application for this trait a basic arithmetic operation like addition would look like the
/// following:
/// ```
/// # use viewbuilder_core as viewbuilder;
/// use viewbuilder::transform::*;
///
/// #[derive(Debug)]
/// struct Add(i32);
///
/// impl OnceTransform<i32> for Add {
///     type Output = i32;
///
///     fn apply_once(self, input: i32) -> Self::Output {
///         self.apply(input)
///     }
/// }
///
/// impl MutableTransform<i32> for Add {
///     fn apply_mut(&mut self, input: i32) -> Self::Output {
///         self.apply(input)
///     }
/// }
///
/// impl Transform<i32> for Add {
///     fn apply(&self, input: i32) -> Self::Output {
///         let Self(value) = self;
///         input + value
///     }
/// }
///
/// fn main() {
///     let mut a = 1;
///     let add1 = Add(1);
///     a = add1.apply(a);
///     println!("{a}");
/// # assert_eq!(a, 2)
/// }
/// ```
/// This trait is analogous to the [`Fn`] trait.
pub trait Transform<Input>
where
    Self: MutableTransform<Input>,
    Input: SendSync,
{
    /// Apply the transform operation consuming the input and yielding the
    /// output.
    fn apply(&self, input: Input) -> Self::Output;
}

/// A mutable transform operation.
///
///
/// ```
/// # use viewbuilder_core as viewbuilder;
/// use viewbuilder::transform::*;
///
/// #[derive(Debug)]
/// struct SequencedAdd(Vec<i32>);
///
/// impl OnceTransform<i32> for SequencedAdd {
///     type Output = i32;
///
///     fn apply_once(mut self, input: i32) -> Self::Output {
///         self.apply_mut(input)
///     }
/// }
///
/// impl MutableTransform<i32> for SequencedAdd {
///     fn apply_mut(&mut self, input: i32) -> Self::Output {
///         let Self(ref mut seq) = self;
///         if let Some(value) = seq.pop() {
///             input + value
///         } else {
///             input
///         }
///     }
/// }
/// fn main() {
///     let mut total = 0;
///     let mut add = SequencedAdd(vec![2, 4]);
///     total = add.apply_mut(total);
///     println!("{total}");
///
///     total = add.apply_mut(total);
///     println!("{total}");
/// # assert_eq!(total, 6);
/// }
/// ```
/// This trait is analogous to the [`FnMut`] trait.
pub trait MutableTransform<Input>
where
    Self: OnceTransform<Input>,
    Input: SendSync,
{
    /// Apply the transform operation consuming the input and yielding the
    /// output.
    fn apply_mut(&mut self, input: Input) -> Self::Output;
}

/// A transform operation that can only happen once.
///
/// ```
/// # use viewbuilder_core as viewbuilder;
/// use viewbuilder::transform::*;
///
/// #[derive(Debug)]
/// struct Add(i32);
///
/// impl OnceTransform<i32> for Add {
///     type Output = i32;
///
///     fn apply_once(self, input: i32) -> Self::Output {
///         let Self(value) = self;
///         input + value
///     }
/// }
///
/// fn main() {
///     let mut a = 1;
///     let add1 = Add(1);
///     a = add1.apply_once(a);
///     println!("{a}");
/// # assert_eq!(a, 2)
/// }
/// ```
/// This trait is analogous to the [`FnOnce`] trait.
pub trait OnceTransform<Input>
where
    Self: SendSync,
    Input: SendSync,
{
    /// the Output type for the transform.
    type Output: SendSync;

    /// Apply the transform operation consuming both the input and transform operation yielding the
    /// output.
    fn apply_once(self, input: Input) -> Self::Output;
}
