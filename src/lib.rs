//! A utility field that can produce a static or random value based on specified parameters that
//! can be utilized in bevy components.

pub mod variable_property;
//pub mod from_vec;
pub mod prop_range;
mod prop_rand;
mod prop_array;

use bevy::prelude::*;

use rand::{
    seq::SliceRandom,
    thread_rng,
};

use std::ops::{Range, RangeInclusive};

use crate::prop_rand::PropRand;
use crate::prop_range::PropRange;
use crate::prop_array::PropArray;

use crate::variable_property::VariableProperty;

/// Generic property that can be static, randomized within a range, randomly selected from a
/// predetermined list, or entirely random on each read.
///
/// Implementation of Default provides `Static(T::default())`
#[derive(Reflect, FromReflect, Clone)]
pub enum Property<T> {
    /// Produces the same value
    Static(T),

    /// Produces a random value within the given range
    RandomRange(PropRange<T>),

    /// Produces a randomly selected value from the given list
    RandomChoice(Vec<T>),

    /// Produces a completely random value
    Random,
}

impl<T, const N: usize> Property<PropArray<T, N>> {
    /// Convenience method for creating a Property::RandomRange from two arrays.
    pub fn from_array_range(start: [T; N], end: [T; N], inclusive: bool) -> Self {
        Property::RandomRange(PropRange { 
            start: start.into(), 
            end: end.into(), 
            inclusive 
        })
    }

    /// Convenience method for creating a Property::RandomChoice from a Vec of arrays.
    pub fn from_array_choices(choices: Vec<[T; N]>) -> Self {
        Property::RandomChoice(choices.into_iter().map(|v| PropArray::from(v)).collect())
    }
}


impl<T> VariableProperty<T> for Property<T>
where
    T: PropRand + Clone 
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


impl<T> From<Range<T>> for Property<T> {
    fn from(value: Range<T>) -> Self {
        Property::RandomRange(value.into())
    }
}

impl<T> From<RangeInclusive<T>> for Property<T> {
    fn from(value: RangeInclusive<T>) -> Self {
        Property::RandomRange(value.into())
    }
}

impl<T> From<Vec<T>> for Property<T> {
    fn from(value: Vec<T>) -> Self {
        Property::RandomChoice(value)
    }
}

impl<T, const N: usize> From<[T; N]> for Property<T> {
    fn from(value: [T; N]) -> Self {
        Property::RandomChoice(value.into())
    }
}

/// Provides `Static(T::default())`
impl<T: Default> Default for Property<T> {
    fn default() -> Self {
        Property::Static(T::default())
    }
}

pub type GenericVecProperty<T, const N: usize> = Property<PropArray<T, N>>;
pub type VecProperty<const N: usize> = GenericVecProperty<f32, N>;
pub type Vec2Property = VecProperty<2>;
pub type Vec3Property = VecProperty<3>;
pub type Vec4Property = VecProperty<4>;


pub mod prelude {
    pub use crate::{Property, variable_property::VariableProperty, prop_range::PropRange, prop_array::PropArray, 
    GenericVecProperty, VecProperty, Vec2Property, Vec3Property, Vec4Property};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_generation() {
        let ranges = (2.5..5.0, -10.0..0.0, 0.0..1.0);
        let vec3_generator: (Property<f32>, Property<f32>, Property<f32>) = (
            ranges.0.clone().into(),
            ranges.1.clone().into(),
            ranges.2.clone().into(),
        );
        let (x,y,z) = vec3_generator.get_value().into();
        assert!(
            ranges.0.contains(&x),
            "{} was not in the range of ({}..{})",
            x,
            ranges.0.start,
            ranges.0.end
        );
        assert!(
            ranges.1.contains(&y),
            "{} was not in the range of ({}..{})",
            y,
            ranges.1.start,
            ranges.1.end
        );
        assert!(
            ranges.2.contains(&z),
            "{} was not in the range of ({}..{})",
            z,
            ranges.2.start,
            ranges.2.end
        );
    }

    #[test]
    fn array_range_generation() {
        let (start, end) = ([0usize, 25], [10, 50]);
        let array_prop = Property::from_array_range(start, end, true);
        let PropArray([x,y]) = array_prop.get_value();
        assert!(
            (start[0]..=end[0]).contains(&x),
            "{} was not in the range of ({}..{})",
            x,
            start[0],
            end[0]
        );
        assert!(
            (start[1]..=end[1]).contains(&y),
            "{} was not in the range of ({}..{})",
            y,
            start[1],
            end[1]
        );
    }


    #[test]
    #[should_panic]
    fn bad_range() {
        let p: Property<f32> = (10.0..1.0).into();
        p.get_value();
    }

    #[test]
    #[should_panic]
    fn bad_array_range() {
        let p: Property<PropArray<f32, 2>> = Property::from_array_range([0.0, 10.0], [1.0, 5.0], false);
        p.get_value();
    }
}
