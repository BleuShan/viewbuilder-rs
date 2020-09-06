//! The core transform operations traits

/// A Transform operation.
///
/// Objects implementing this trait should represent a transform operation over an arbitrary domain.
/// An application for this trait a basic arithmetic operation like addition would look like the
/// following:
/// ```
/// use viewbuilder_core::transform::*;
///
/// #[repr(transparent)]
/// #[derive(Debug)]
/// pub struct Add(i32);
///
/// impl Add {
///     pub fn new(n: i32) -> Self {
///         Self(n)
///     }
///
///     pub fn value(&self) -> i32 {
///         self.0
///     }
/// }
///
/// impl Transform for Add {
///     type Input = i32;
///     type Output = i32;
///
///     fn apply(&self, input: Self::Input) -> Self::Output {
///         input + self.value()
///     }
/// }
///
/// fn main() {
///     let mut a = 1;
///     let add1 = Add::new(1);
///     a = add1.apply(a);
///     println!("{a}");
/// # assert_eq!(a, 2)
/// }
/// ```
pub trait Transform
where
    Self: Sized + Send + Sync,
{
    /// the Input type for the transform.
    type Input;
    /// the Output type for the transform.
    type Output;

    /// Apply the transform operation consuming the input and yielding the
    /// output.
    fn apply(&self, input: Self::Input) -> Self::Output;
}

/// A Transform operation with an inverse relation ship with another tranform type.
///
/// The intended use of this trait is to establish the relationship between a transform and its
/// inverse. An implementation for addition and substraction this would look like:
/// ```
/// use viewbuilder_core::transform::*;
///
/// #[repr(transparent)]
/// #[derive(Debug)]
/// pub struct Add(i32);
///
/// impl Add {
///     pub fn new(n: i32) -> Self {
///         Self(n)
///     }
///     pub fn value(&self) -> i32 {
///         self.0
///     }
/// }
///
/// impl Transform for Add {
///     type Input = i32;
///     type Output = i32;
///
///     fn apply(&self, input: Self::Input) -> Self::Output {
///         input + self.value()
///     }
/// }
/// impl InversibleTransform for Add {
///     type Inverse = Sub;
///     fn invert(&self) -> Sub {
///         Sub::new(self.value())
///     }
/// }
///
/// #[repr(transparent)]
/// #[derive(Debug)]
/// pub struct Sub(i32);
///
/// impl Sub {
///     pub fn new(n: i32) -> Self {
///         Self(n)
///     }
///
///     pub fn value(&self) -> i32 {
///         self.0
///     }
/// }
///
/// impl Transform for Sub {
///     type Input = i32;
///     type Output = i32;
///
///     fn apply(&self, input: Self::Input) -> Self::Output {
///         input - self.value()
///     }
/// }
///
/// fn main() {
///     let mut a = 0;
///     let add1 = Add::new(1);
///     a = add1.apply(a);
///     a = add1.invert().apply(a);
///     println!("{a}");
/// # assert_eq!(a, 0);
/// }
/// ```
pub trait InversibleTransform
where
    Self: Transform,
{
    /// The [`Transform`] type that correspond to the inverse operation.
    type Inverse: Transform;

    /// Create an instance of the [`InversibleTransform::Inverse`]
    fn invert(&self) -> Self::Inverse;
}

/// A Transform operation that can be reverted.
///
/// Objects implementing this trait should implement the exact opposite of their
/// [`Transform::apply`] operation. For addition and substraction this would
/// look like:
/// ```
/// use viewbuilder_core::transform::*;
///
/// #[repr(transparent)]
/// #[derive(Debug)]
/// pub struct Add(i32);
///
/// impl Add {
///     pub fn new(n: i32) -> Self {
///         Self(n)
///     }
///     pub fn value(&self) -> i32 {
///         self.0
///     }
/// }
///
/// impl Transform for Add {
///     type Input = i32;
///     type Output = i32;
///
///     fn apply(&self, input: Self::Input) -> Self::Output {
///         input + self.value()
///     }
/// }
/// impl InversibleTransform for Add {
///     type Inverse = Sub;
///     fn invert(&self) -> Sub {
///         Sub::new(self.value())
///     }
/// }
///
/// #[repr(transparent)]
/// #[derive(Debug)]
/// pub struct Sub(i32);
///
/// impl Sub {
///     pub fn new(n: i32) -> Self {
///         Self(n)
///     }
///
///     pub fn value(&self) -> i32 {
///         self.0
///     }
/// }
///
/// impl Transform for Sub {
///     type Input = i32;
///     type Output = i32;
///
///     fn apply(&self, input: Self::Input) -> Self::Output {
///         input - self.value()
///     }
/// }
///
/// impl RevertableTransform for Sub {
///     fn revert(&self, input: Self::Output) -> Self::Input {
///         input + self.value()
///     }
/// }
///
/// fn add1_and_revert(n: i32) -> i32 {
///     let add1 = Add::new(1);
///     let result = add1.apply(n);
///     add1.revert(result)
/// }
///
/// fn sub1_and_revert(n: i32) -> i32 {
///     let sub1 = Sub::new(1);
///     let result = sub1.apply(n);
///     sub1.revert(result)
/// }
///
/// fn main() {
///     let a = add1_and_revert(0);
///     let b = sub1_and_revert(1);
///     println!("a: {a}, b: {b}");
/// # assert_eq!(a, 0);
/// # assert_eq!(b, 1);
/// }
/// ```
/// A blanket implementation of this provided for any [`InversibleTransform`] where the
/// [`InversibleTransform::Inverse`] input and output type matches the
/// [`RevertableTransform::revert`] signature.
pub trait RevertableTransform
where
    Self: Transform,
{
    /// Apply the inverse of the [`Transform::apply`] operation.
    fn revert(&self, input: Self::Output) -> Self::Input;
}

impl<InversibleType> RevertableTransform for InversibleType
where
    InversibleType: InversibleTransform,
    <Self as InversibleTransform>::Inverse: Transform<Input = Self::Output, Output = Self::Input>,
{
    fn revert(&self, input: Self::Output) -> Self::Input {
        self.invert().apply(input)
    }
}
