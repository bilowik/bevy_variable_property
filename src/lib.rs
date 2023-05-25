//! A utility field that can produce a static or random value based on specified parameters that
//! can be utilized in bevy components.

use bevy::prelude::*;

use rand::{
    seq::SliceRandom,
    thread_rng,
};

use std::ops::{Range, RangeInclusive};

pub mod variable_property;
//pub mod from_vec;
pub mod prop_range;
mod prop_rand;

//use prop_range::{PropArray, PropRange};
use prop_rand::PropRand;
use prop_range::{PropRange, PropArray};

use variable_property::VariableProperty;

/// Generic property that can be static, randomized within a range, randomly selected from a
/// predetermined list, or entirely random on each read.
///
/// Implementation of Default provides `Static(T::default())`
#[derive(Reflect, FromReflect, Clone)]
pub enum Property<T>
where
    T: PropRand + Send + Sync + 'static + Reflect + FromReflect,
{
    /// Produces the same value
    Static(T),

    /// Produces a random value within the given range
    RandomRange(PropRange<T>),

    /// Produces a randomly selected value from the given list
    RandomChoice(Vec<T>),

    /// Produces a completely random value
    Random,
}

impl<T> VariableProperty<T> for Property<T>
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    /// Gets a value based on the parameters of the Property
    /// See [Property] for more information.
    fn get_value(&self) -> T {
        match self {
            Property::Static(v) => v.clone(),
            Property::RandomRange(range) => <T as PropRand>::gen_range(&mut thread_rng(), range.clone()),
            Property::RandomChoice(choices) => choices.choose(&mut thread_rng()).unwrap().clone(),
            Property::Random => T::gen(&mut thread_rng()),
        }
    }
}

impl<T> From<T> for Property<T>
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    fn from(value: T) -> Self {
        Property::Static(value)
    }
}

impl<T> From<Range<T>> for Property<T>
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    fn from(value: Range<T>) -> Self {
        Property::RandomRange(value.into())
    }
}

impl<T, const N: usize> From<Range<[T; N]>> for Property<PropArray<T, N>> 
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    fn from(value: Range<[T; N]>) -> Self {
        Property::RandomRange(value.into())
    }
}

impl<T> From<RangeInclusive<T>> for Property<T>
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    fn from(value: RangeInclusive<T>) -> Self {
        Property::RandomRange(value.into())
    }
}

impl<T, const N: usize> From<RangeInclusive<[T; N]>> for Property<PropArray<T, N>> 
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    fn from(value: RangeInclusive<[T; N]>) -> Self {
        Property::RandomRange(value.into())
    }
}

impl<T> From<Vec<T>> for Property<T>
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    fn from(value: Vec<T>) -> Self {
        Property::RandomChoice(value)
    }
}

impl<T, const N: usize> From<[T; N]> for Property<T>
where
    T: PropRand + Clone + Reflect + FromReflect,
{
    fn from(value: [T; N]) -> Self {
        Property::RandomChoice(value.into())
    }
}

/// Provides `Static(T::default())`
impl<T> Default for Property<T>
where
    T: PropRand + Clone + Reflect + FromReflect + Default,
{
    fn default() -> Self {
        T::default().into()
    }
}


pub mod prelude {
    pub use crate::{Property, variable_property::VariableProperty, prop_range::{PropArray, PropRange}};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prop_range::*;
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


    #[test]
    #[should_panic]
    fn bad_range() {
        let p: Property<f32> = (10.0..1.0).into();
        p.get_value();

    }

    #[test]
    fn vecs() {
        let p: Property<PropArray<f32, 2>> = ([0.0, 20.0]..[10.0, 30.0]).into();
        for _ in 0..10 {
            println!("{:?}", p.get_value());
        }

    }
}
