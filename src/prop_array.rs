use bevy::prelude::*;


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

