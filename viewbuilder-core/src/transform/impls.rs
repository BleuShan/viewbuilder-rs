//! [`transform`](crate::transform) module default implementations.

use super::*;

impl<Input, TransformType> Transform<Input> for &TransformType
where
    TransformType: Transform<Input>,
    Input: SendSync,
{
    fn apply(&self, input: Input) -> Self::Output {
        (**self).apply(input)
    }
}

impl<Input, TransformType> MutableTransform<Input> for &TransformType
where
    TransformType: Transform<Input>,
    Input: SendSync,
{
    fn apply_mut(&mut self, input: Input) -> Self::Output {
        (**self).apply(input)
    }
}

impl<Input, TransformType> OnceTransform<Input> for &TransformType
where
    TransformType: Transform<Input>,
    Input: SendSync,
{
    type Output = TransformType::Output;

    fn apply_once(self, input: Input) -> Self::Output {
        (*self).apply(input)
    }
}

impl<Input, TransformType> MutableTransform<Input> for &mut TransformType
where
    TransformType: MutableTransform<Input>,
    Input: SendSync,
{
    fn apply_mut(&mut self, input: Input) -> Self::Output {
        (**self).apply_mut(input)
    }
}

impl<Input, TransformType> OnceTransform<Input> for &mut TransformType
where
    TransformType: MutableTransform<Input>,
    Input: SendSync,
{
    type Output = TransformType::Output;

    fn apply_once(self, input: Input) -> Self::Output {
        (*self).apply_mut(input)
    }
}
