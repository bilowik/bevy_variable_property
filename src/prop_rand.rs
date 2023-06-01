use bevy::math::*;

use array_macro::array;

use rand::{
    Rng, 
    RngCore,
    distributions::{Standard, Distribution},
};

use crate::prop_range::PropRange;

pub trait PropRand {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self;
    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self
    where
        Self: Sized;
}

macro_rules! prop_rand_impl {
    ($type:tt) => {
        impl PropRand for $type {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                rng.gen()
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<$type>) -> Self {
                if range.inclusive {
                    rng.gen_range(range.start..=range.end)
                } else {
                    rng.gen_range(range.start..range.end)
                }
            }
        }
    };
}

macro_rules! prop_rand_impl_many {
    ($($type:tt,)+) => {
        $(
            prop_rand_impl!($type);
        )+
    };
}

prop_rand_impl_many!(usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64,);

impl<T, const N: usize> PropRand for [T; N]
where
    T: PropRand + Clone
{
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        array![_ => T::gen(rng); N].into()
    }

    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<[T; N]>) -> Self {
        //Seems like it could be sluggish sampling each time, there might be a better way to implement this.
        if range.inclusive {
            array![i => T::gen_range(rng, (range.start[i].clone()..=range.end[i].clone()).into()); N].into()
        } else {
            array![i => T::gen_range(rng, (range.start[i].clone()..range.end[i].clone()).into()); N]
                .into()
        }
    }
}

macro_rules! prop_rand_tuple_impls_inner {
    () => {};
    ($range:ident, $rng:ident, [$($list_idx:literal $list:tt,)*], $head_idx:literal $head:tt, $($tail_idx:literal $tail:tt,)*) => {
       prop_rand_tuple_impls_inner!($range, $rng, [$head_idx $head, $($list_idx $list,)*], $($tail_idx $tail,)*)
    };
    ($range:ident, $rng:ident, [$($idx:literal $list:tt,)+],) => {
        paste::paste! {(
            $($list::gen_range($rng, PropRange { start: $range.start.$idx, end: $range.end.$idx, inclusive: $range.inclusive }),)+
        )}
    }
}

macro_rules! prop_rand_tuple_impls_inner_2 {
    () => {};
    ([$($type_in_list:tt,)*], $head_type:tt, $($tail:tt,)*) => {
        prop_rand_tuple_impls_inner_2!([$head_type, $($type_in_list,)*], $($tail,)*)
    };
    ([$($type_in_list:tt,)+],) => {
        ($($type_in_list,)+)
    };
}

macro_rules! prop_rand_tuple_impls_inner_3 {
    () => {};
    ($rng:ident, [$($list_idx:literal $list:tt,)*], $head_idx:literal $head:tt, $($tail_idx:literal $tail:tt,)*) => {
       prop_rand_tuple_impls_inner_3!($rng, [$head_idx $head, $($list_idx $list,)*], $($tail_idx $tail,)*)
    };
    ($rng:ident, [$($idx:literal $list:tt,)+],) => {
        paste::paste! {(
            $($list::gen($rng),)+
        )}
    }
}

macro_rules! prop_rand_tuple_impls {
    () => {};
    ($head_idx:literal $head:tt, $($tail_idx:literal $tail:tt,)*) => {
        impl<$head, $($tail,)*> PropRand for prop_rand_tuple_impls_inner_2!([], $head, $($tail,)*)
            where $head: PropRand + Clone, $($tail: PropRand + Clone,)*
        {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                prop_rand_tuple_impls_inner_3!(rng, [], $head_idx $head, $($tail_idx $tail,)*)
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<prop_rand_tuple_impls_inner_2!([], $head, $($tail,)*)>) -> Self {
                prop_rand_tuple_impls_inner!(range, rng, [], $head_idx $head, $($tail_idx $tail,)*)

            }
        }

        prop_rand_tuple_impls!($($tail_idx $tail,)*);

    }
}

prop_rand_tuple_impls!(15 P, 14 O, 13 N, 12 M, 11 L, 10 K, 9 J, 8 I, 7 H, 6 G, 5 F, 4 E, 3 D, 2 C, 1 B, 0 A,);

// The reason we don't just use a version of the top impl of the macro is for efficiency.
// There's 3 conversiosn being done per call.
//
// NOTE: If above we required Standard: Distribution for [T; N], then another performance
// impliciation to think about is using <[T; N]>::gen(rng).into() so we would only sample
// the rng once (I think?) and that could make the first impl below the most performant
// potentially.
macro_rules! prop_rand_vec_impl {
    ($vec_type:tt, $inner_type:tt, $size:literal) => {
        impl PropRand for $vec_type {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                <[$inner_type; $size]>::gen(rng).into()
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self {
                <[$inner_type; $size]>::gen_range(rng, PropRange {
                    start: range.start.into(),
                    end: range.end.into(),
                    inclusive: range.inclusive
                }).into()
            }
        }
    };
}

prop_rand_vec_impl!(Vec2, f32, 2);
prop_rand_vec_impl!(Vec3, f32, 3);
prop_rand_vec_impl!(Vec4, f32, 4);

prop_rand_vec_impl!(DVec2, f64, 2);
prop_rand_vec_impl!(DVec3, f64, 3);
prop_rand_vec_impl!(DVec4, f64, 4);

prop_rand_vec_impl!(UVec2, u32, 2);
prop_rand_vec_impl!(UVec3, u32, 3);
prop_rand_vec_impl!(UVec4, u32, 4);

prop_rand_vec_impl!(IVec2, i32, 2);
prop_rand_vec_impl!(IVec3, i32, 3);
prop_rand_vec_impl!(IVec4, i32, 4);

impl PropRand for Rect {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        Rect::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self {
        Rect {
            min: Vec2::gen_range(
                rng,
                PropRange {
                    start: range.start.min,
                    end: range.end.min,
                    inclusive: range.inclusive,
                },
            ),
            max: Vec2::gen_range(
                rng,
                PropRange {
                    start: range.start.max,
                    end: range.end.max,
                    inclusive: range.inclusive,
                },
            ),
        }
    }
}

/*impl PropRand for Vec2 {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        rng.gen::<(f32, f32)>().into()
    }
    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self {
        <(f32, f32)>::gen_range(rng, PropRange { start: range.start.into(), end: range.end.into(), inclusive: range.inclusive }).into()
    }
}*/
