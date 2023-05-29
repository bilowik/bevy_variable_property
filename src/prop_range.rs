use bevy::prelude::*;



use std::ops::{Range, RangeInclusive};

/// Wrapper around Range<T> for foreign trait implementations
#[derive(Clone, Default, Debug, Reflect, FromReflect)]
pub struct PropRange<T> {
    pub start: T,
    pub end: T,
    pub inclusive: bool,
}

impl<T> PropRange<T> {
    pub fn new(start: T, end: T, inclusive: bool) -> Self {
        Self {
            start,
            end,
            inclusive,
        }
    }
}



impl<T> From<Range<T>> for PropRange<T> {
    fn from(v: Range<T>) -> Self {
        Self {
            start: v.start,
            end: v.end,
            inclusive: false,
        }
    }
}


impl<T> From<RangeInclusive<T>> for PropRange<T> {
    fn from(v: RangeInclusive<T>) -> Self {
        let (start, end) = v.into_inner();
        Self {
            start,
            end, 
            inclusive: true,
        }
    }
}



/*impl<T: PartialOrd + SampleUniform + Copy, const N: usize> SampleRange<[T; N]> for PropRange<[T; N]> {
    fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> [T; N] {
        array![i => (self.0.start[i]..self.0.end[i]).sample_single(rng); N]
    }
    
    fn is_empty(&self) -> bool {
        return self.0.start
            .as_slice()
            .into_iter()
            .zip(self.0.end.as_slice().into_iter())
            .any(|(s, e)| s >= e);
    }
}


#[derive(Clone, Copy, Debug)]
pub struct UniformFloatArray<T, const N: usize>([UniformFloat<T>; N]);

impl<T, const N: usize> From<[UniformFloat<T>; N]> for UniformFloatArray<T, N> {
	fn from(other: [UniformFloat<T>; N]) -> Self {
		Self(other)
	}
}


macro_rules! impl_uniform_float_sampler {
	($float:ty) => {
		impl<const N: usize> UniformSampler for UniformFloatArray<$float, N> {
			type X = PropArray<$float, N>;

			fn new<B1, B2>(low: B1, high: B2) -> Self 
				where B1: SampleBorrow<Self::X> + Sized, B2: SampleBorrow<Self::X> + Sized {
				UniformFloatArray::<$float, N>(array![i => UniformFloat::<$float>::new(low.borrow()[i], high.borrow()[i]); N]) 
			}
			fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self 
				where B1: SampleBorrow<Self::X> + Sized, B2: SampleBorrow<Self::X> + Sized {
				array![i => UniformFloat::<$float>::new_inclusive(low.borrow()[i], high.borrow()[i]); N].into()
			}

			fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
				array![i => self.0[i].sample(rng); N].into()
			}
		}
	}
}

impl_uniform_float_sampler!(f32);
impl_uniform_float_sampler!(f64);


impl<const N: usize> SampleUniform for PropArray<f32, N> {
    type Sampler = UniformFloatArray<f32, N>;
}

impl<const N: usize> SampleUniform for PropArray<f64, N> {
    type Sampler = UniformFloatArray<f64, N>;
}




impl<T, const N: usize> Distribution<PropArray<T, N>> for Standard 
where Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PropArray<T, N> {
        array![_ => rng.gen(); N].into()
    }
}

*/
