//! A utility field that can produce a static or random value based on specified parameters that
//! can be utilized in bevy components.

pub mod interval_property;
pub mod prop_rand;
pub mod prop_range;
pub mod variable_property;

use bevy::{
    math::{DVec2, DVec3, DVec4},
    prelude::*,
};

use rand::{seq::SliceRandom, thread_rng};

use std::ops::{Range, RangeInclusive};

use crate::prop_rand::PropRand;
use crate::prop_range::PropRange;

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

impl<T> VariableProperty for Property<T>
where
    T: PropRand + Clone,
{
    type Output = T;
    /// Gets a value based on the parameters of the Property
    /// See [Property] for more information.
    fn get_value(&self) -> T {
        match self {
            Property::Static(v) => v.clone(),
            Property::RandomRange(range) => {
                <T as PropRand>::gen_range(&mut thread_rng(), range.clone())
            }
            Property::RandomChoice(choices) => choices.choose(&mut thread_rng()).unwrap().clone(),
            Property::Random => T::gen(&mut thread_rng()),
        }
    }
}

/// Provides `Static(T::default())`
impl<T: Default> Default for Property<T> {
    fn default() -> Self {
        Property::Static(T::default())
    }
}

/// Implements a variety of From implementations for the given type.
///
/// From<$type> -> Static, From<Range<$type>> -> RandomRange, From<RangeInclusve<$type>> -> RandomRange,
/// From<Vec<$type>> -> RandomChoice, and From<&[$type]> -> RandomChoice,
///
/// The $from_type **must** implement Clone and PropRand to be able to utilize [Property::get_value].
///
/// If two types are provided, $from_type must implement Into for $into_prop_type
macro_rules! prop_from_impl {
    ($from_type:tt) => {
        prop_from_impl!($from_type, $from_type);
    };

    ($from_type:tt, $into_prop_type:tt) => {
        impl From<$from_type> for Property<$into_prop_type> {
            fn from(v: $from_type) -> Self {
                Property::Static(v.into())
            }
        }

        impl From<Range<$from_type>> for Property<$into_prop_type> {
            fn from(v: Range<$from_type>) -> Self {
                Property::RandomRange(PropRange {
                    start: v.start.into(),
                    end: v.end.into(),
                    inclusive: false,
                })
            }
        }

        impl From<RangeInclusive<$from_type>> for Property<$into_prop_type> {
            fn from(v: RangeInclusive<$from_type>) -> Self {
                Property::RandomRange(PropRange {
                    start: v.start().clone().into(),
                    end: v.end().clone().into(),
                    inclusive: true,
                })
            }
        }

        impl From<Vec<$from_type>> for Property<$into_prop_type> {
            fn from(v: Vec<$from_type>) -> Self {
                Property::RandomChoice(v.into_iter().map(|x| x.into()).collect())
            }
        }

        impl From<&[$from_type]> for Property<$into_prop_type> {
            fn from(v: &[$from_type]) -> Self {
                Property::RandomChoice(v.into_iter().cloned().map(|x| x.into()).collect())
            }
        }

        impl<const N: usize> From<[$from_type; N]> for Property<$into_prop_type> {
            fn from(v: [$from_type; N]) -> Self {
                Property::RandomChoice(v.into())
            }
        }
    };
}

/// Implements a variety of From implementations for the given type.
///
/// From<$type> -> Static, From<Range<$type>> -> RandomRange, From<RangeInclusve<$type>> -> RandomRange,
/// From<Vec<$type>> -> RandomChoice, and From<&[$type]> -> RandomChoice,
///
/// The $from_type **must** implement Clone and PropRand to be able to utilize [Property::get_value].
macro_rules! prop_from_impl_many {
    ($($type:tt,)+) => {
        $(
            prop_from_impl!($type, $type);
        )+
    }
}

prop_from_impl_many!(
    usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, Vec2, Vec3, Vec4,
    UVec2, UVec3, UVec4, IVec2, IVec3, IVec4, DVec2, DVec3, DVec4,
);

impl<T, const N: usize> From<Range<[T; N]>> for Property<[T; N]> {
    fn from(v: Range<[T; N]>) -> Self {
        Self::RandomRange(PropRange {
            start: v.start.into(),
            end: v.end.into(),
            inclusive: false,
        })
    }
}

impl<T: Clone, const N: usize> From<RangeInclusive<[T; N]>> for Property<[T; N]> {
    fn from(v: RangeInclusive<[T; N]>) -> Self {
        Self::RandomRange(PropRange {
            start: v.start().clone(),
            end: v.end().clone(),
            inclusive: true,
        })
    }
}

impl<T, const N: usize> From<Vec<[T; N]>> for Property<[T; N]> {
    fn from(v: Vec<[T; N]>) -> Self {
        Property::RandomChoice(v.into_iter().map(|x| x.into()).collect())
    }
}

impl<T: Clone, const N: usize> From<&[[T; N]]> for Property<[T; N]> {
    fn from(v: &[[T; N]]) -> Self {
        Property::RandomChoice(v.into_iter().cloned().collect())
    }
}

impl<T, const N: usize, const M: usize> From<[[T; N]; M]> for Property<[T; N]> {
    fn from(v: [[T; N]; M]) -> Self {
        Property::RandomChoice(v.into_iter().collect())
    }
}

pub mod prelude {
    pub use crate::{
        interval_property::IntervalProperty, prop_range::PropRange,
        variable_property::VariableProperty, Property,
    };
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
        let (x, y, z) = vec3_generator.get_value().into();
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
        let array_prop: Property<[usize; 2]> = (start..=end).into();
        let [x, y] = array_prop.get_value();
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
        let p = Property::from(10.0..1.0);
        p.get_value();
    }

    #[test]
    #[should_panic]
    fn bad_array_range() {
        let p = Property::from([0.0, 10.0]..[1.0, 5.0]);
        p.get_value();
    }

    #[test]
    fn tuples() {
        let p = Property::Static((1.0, 5.0));
        p.get_value();
    }
}
