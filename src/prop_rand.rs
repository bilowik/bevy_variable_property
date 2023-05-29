use bevy::prelude::*;

use array_macro::array;

use rand::{
    Rng,
    RngCore,
};

use crate::prop_range::PropRange;



pub trait PropRand {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self;
    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self
        where Self: Sized; }

macro_rules! prop_rand_impl {
    ($type:tt) => {
        impl PropRand for $type  {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                rng.gen()
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<$type>) -> Self {
                if range.inclusive {
                    rng.gen_range(range.start..=range.end)
                }
                else {
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
where T: PropRand + Clone {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        array![_ => T::gen(rng); N]
    }

    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<[T; N]>) -> Self {
        if range.inclusive {
            array![i => T::gen_range(rng, (range.start[i].clone()..=range.end[i].clone()).into()); N].into()
        }
        else {
            array![i => T::gen_range(rng, (range.start[i].clone()..range.end[i].clone()).into()); N].into()
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


macro_rules! prop_rand_vec_impls_old {
    () => {};
    ($head_vec:tt $head_tuple_equivalent:tt, $($tail_vec:tt $tail_tuple_equivalent:tt,)*) => {
        impl PropRand for $head_vec {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                rng.gen::<$head_tuple_equivalent>().into()
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self {
                <$head_tuple_equivalent>::gen_range(rng, PropRange { start: range.into(), end: range.into(), inclusive: range.inclusive })
            }
        }

        prop_rand_vec_impls!($($tail_vec $tail_tuple_equivalent,)*)
    };
}

macro_rules! prop_rand_vec_impl {
    ($vec_type:tt, $inner_type:tt, $($dim_ident:ident,)+) => {
        impl PropRand for $vec_type {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                $vec_type {
                    $($dim_ident: rng.gen(),)+
                }
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self {
                $vec_type {
                    $($dim_ident: $inner_type::gen_range(rng, PropRange { start: range.start.$dim_ident, end: range.end.$dim_ident, inclusive: range.inclusive }),)+
                }
            }
        }
    }
}


prop_rand_vec_impl!(Vec2, f32, x, y,);
prop_rand_vec_impl!(Vec3, f32, x, y, z,);
prop_rand_vec_impl!(Vec4, f32, x, y, z, w,);


/*impl PropRand for Vec2 {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        rng.gen::<(f32, f32)>().into()
    }
    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self {
        <(f32, f32)>::gen_range(rng, PropRange { start: range.start.into(), end: range.end.into(), inclusive: range.inclusive }).into()
    }
}*/

