use bevy::prelude::*;

use rand::{
    distributions::{uniform::SampleUniform, Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};

use std::ops::Range;

mod variable_property;

use variable_property::VariableProperty;

/// Generic property that can be static, randomized within a range, randomly selected from a
/// predetermined list, or entirely random on each read.
///
/// Implementation of Default provides `Static(T::default())`
#[derive(Reflect, FromReflect, Clone)]
pub enum Property<T>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    /// Produces the same value
    Static(T),

    /// Produces a random value within the given range
    RandomRange(Range<T>),

    /// Produces a randomly selected value from the given list
    RandomChoice(Vec<T>),

    /// Produces a completely random value
    Random,
}

impl<T> VariableProperty<T> for Property<T>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    /// Gets a value based on the parameters of the Property
    /// See [Property] for more information.
    fn get_value(&self) -> T {
        match self {
            Property::Static(v) => v.clone(),
            Property::RandomRange(range) => thread_rng().gen_range(range.clone()),
            Property::RandomChoice(choices) => choices.choose(&mut thread_rng()).unwrap().clone(),
            Property::Random => thread_rng().gen(),
        }
    }
}

impl<T> From<T> for Property<T>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    fn from(value: T) -> Self {
        Property::Static(value)
    }
}

impl<T> From<Range<T>> for Property<T>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    fn from(value: Range<T>) -> Self {
        Property::RandomRange(value)
    }
}

impl<T> From<Vec<T>> for Property<T>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    fn from(value: Vec<T>) -> Self {
        Property::RandomChoice(value)
    }
}

impl<T, const N: usize> From<[T; N]> for Property<T>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    fn from(value: [T; N]) -> Self {
        Property::RandomChoice(value.into())
    }
}

/// Provides `Static(T::default())`
impl<T> Default for Property<T>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    fn default() -> Self {
        T::default().into()
    }
}

#[cfg(test)]
mod tests {
    use super::{Property, VariableProperty};
    use bevy::prelude::*;
    #[test]
    fn range_generation() {
        let ranges = (2.5..5.0, -10.0..0.0, 0.0..1.0);
        let vec3_generator: (Property<f32>, Property<f32>, Property<f32>) = (
            ranges.0.clone().into(),
            ranges.1.clone().into(),
            ranges.2.clone().into(),
        );
        for _ in 0..10 {
            let v: Vec3 = vec3_generator.get_value().into();
            assert!(
                ranges.0.contains(&v.x),
                "{} was not in the range of ({}..{})",
                v.x,
                ranges.0.start,
                ranges.0.end
            );
            assert!(
                ranges.1.contains(&v.y),
                "{} was not in the range of ({}..{})",
                v.y,
                ranges.1.start,
                ranges.1.end
            );
            assert!(
                ranges.2.contains(&v.z),
                "{} was not in the range of ({}..{})",
                v.z,
                ranges.2.start,
                ranges.2.end
            );
        }
    }
}
