use array_macro::array;
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

// The reason Vec2Property and Vec3Property don't utilize VecProperty internally is mainly for
// efficiency and ease of access and mutation.

/// A Vec2 where x and y are `Property<f32>`, see [Property] for more information.
#[derive(Reflect, FromReflect, Clone, Default)]
pub struct Vec2Property {
    pub x: Property<f32>,
    pub y: Property<f32>,
}

impl Vec2Property {
    pub fn new(x: Property<f32>, y: Property<f32>) -> Self {
        Self { x, y }
    }

    /// Generates a Vec2 from the set properties
    pub fn get(&self) -> Vec2 {
        Vec2::new(self.x.get_value(), self.y.get_value())
    }
}

/// A Vec3 where x and y are `Property<f32>`, see [Property] for more information.
#[derive(Reflect, FromReflect, Clone, Default)]
pub struct Vec3Property {
    pub x: Property<f32>,
    pub y: Property<f32>,
    pub z: Property<f32>,
}

impl Vec3Property {
    pub fn new(x: Property<f32>, y: Property<f32>, z: Property<f32>) -> Self {
        Self { x, y, z }
    }

    /// Generates a Vec3 from the set properties
    pub fn get(&self) -> Vec3 {
        Vec3::new(self.x.get_value(), self.y.get_value(), self.z.get_value())
    }
}

/// A generic Vec-generating struct if Vec2Property and Vec3Property do not fit your needs.
/// See [Property] for more information.
///
/// For example, if you a field that generates a UVec2, you could utilize this like so:
/// ```no_run
/// struct MyStruct {
///     _vec: VecProperty<u32, 2>,
/// }
///
/// // Getting the UVec2 is simple since all of the Vec types utilized in Bevy support
/// // From arrays.
///
/// impl MyStruct {
///     pub fn vec() -> UVec2 {
///         self._vec.get().into()
///     }
/// }
/// ```
#[derive(Reflect, FromReflect, Clone)]
pub struct VecProperty<T, const N: usize>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    pub properties: [Property<T>; N],
}

impl<const N: usize, T> VecProperty<T, N>
where
    T: Clone + PartialOrd + SampleUniform + Send + Sync + Default + 'static + Reflect + FromReflect,
    Standard: Distribution<T>,
{
    pub fn new(properties: [Property<T>; N]) -> Self {
        Self { properties }
    }

    /// Creates an array of T with size N from the given properties.
    /// Can convert to one of bevy's Vec types if applicable with `into()`
    pub fn get(&self) -> [T; N] {
        array![i => self.properties[i].get_value(); N]
    }
}

/// A Property-like color field that can generate random colors, select one from a set list, or
/// provide a static one.
#[derive(Reflect, FromReflect)]
pub enum ColorGenerator {
    Static(Color),
    Range(VecProperty<f32, 4>),
    RandomFromList(Vec<Color>),
    Random,
}

impl ColorGenerator {
    pub fn generate(&self) -> Color {
        match self {
            ColorGenerator::Static(color) => *color,
            ColorGenerator::Range(v) => v.get().into(),
            ColorGenerator::RandomFromList(colors) => *colors.choose(&mut thread_rng()).unwrap(),
            ColorGenerator::Random => {
                let mut rng = thread_rng();
                Color::rgb_u8(rng.gen(), rng.gen(), rng.gen())
            }
        }
    }
}
