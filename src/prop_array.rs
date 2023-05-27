use bevy::{
    prelude::*,
    math::*,
};


/// Wrapper around generic-length array for foreign trait implementations
#[derive(Clone, Debug, Reflect, FromReflect)]
pub struct PropArray<T, const N: usize>(pub [T; N]);


impl<T: Default + Copy, const N: usize> Default for PropArray<T, N> {
    fn default() -> Self {
        PropArray([T::default(); N])
    }
}

impl<T, const N: usize> std::ops::Deref for PropArray<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> std::ops::DerefMut for PropArray<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl<T, const N: usize> From<[T; N]> for PropArray<T, N> {
    fn from(other: [T; N]) -> Self {
        Self(other)
    }
}

impl<T, const N: usize> Into<[T; N]> for PropArray<T, N> {
    fn into(self) -> [T; N] {
        self.0
    }
}

macro_rules! prop_array_into_impl {
    ($vec_type:tt, $element_type:tt, $dim:literal) => {
        impl Into<$vec_type> for PropArray<$element_type, $dim> {
            fn into(self) -> $vec_type {
                self.0.into()
            }
        }
        
    };
}


prop_array_into_impl!(Vec2, f32, 2);
prop_array_into_impl!(Vec3, f32, 3);
prop_array_into_impl!(Vec4, f32, 4);

prop_array_into_impl!(DVec2, f64, 2);
prop_array_into_impl!(DVec3, f64, 3);
prop_array_into_impl!(DVec4, f64, 4);

prop_array_into_impl!(UVec2, u32, 2);
prop_array_into_impl!(UVec3, u32, 3);
prop_array_into_impl!(UVec4, u32, 4);

prop_array_into_impl!(IVec2, i32, 2);
prop_array_into_impl!(IVec3, i32, 3);
prop_array_into_impl!(IVec4, i32, 4);

prop_array_into_impl!(Vec3A, f32, 3);


prop_array_into_impl!(Color, f32, 4);
