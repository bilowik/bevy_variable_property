use array_macro::array;

use rand::{
    Rng,
    RngCore,
    distributions::{
        uniform::SampleUniform,
        Standard,
        Distribution,
    },
};

use crate::prop_range::PropRange;

use crate::prop_array::PropArray;


pub trait PropRand {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self;
    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<Self>) -> Self
        where Self: Sized;
}

impl<T> PropRand for T 
where T: PartialOrd + SampleUniform, Standard: Distribution<T> {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        rng.gen()
    }

    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<T>) -> Self {
        if range.inclusive {
            rng.gen_range(range.start..=range.end)
        }
        else {
            rng.gen_range(range.start..range.end)
        }

    }
}

impl<T, const N: usize> PropRand for PropArray<T, N> 
where T: PropRand + Clone {
    fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
        array![_ => T::gen(rng); N].into()
    }

    fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<PropArray<T, N>>) -> Self {
        if range.inclusive {
            array![i => T::gen_range(rng, (range.start.0[i].clone()..=range.end.0[i].clone()).into()); N].into()
        }
        else {
            array![i => T::gen_range(rng, (range.start.0[i].clone()..range.end.0[i].clone()).into()); N].into()
        }
    }
}


/* Same issue as below, the PartialOrd bound on the above makes this not work unfortunately. 
macro_rules! prop_rand_tuple_impls {
    () => {};
    ($head_idx:tt $head:tt, $($idx:tt $tail:tt,)+) => {
        impl<$head, $($tail,)+> PropRand for ($head, $($tail,)+) 
            where $head: PropRand + Clone, $($tail: PropRand + Clone),+
        {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                ($head::gen(rng), $($tail::gen(rng),)+)
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<($head, $($tail,)+)>) -> Self {
                ($head::gen_range(rng, range.start.0.clone()..range.end.0.clone()), 
                 $($tail::gen_range(rng, range.start.$idx.clone()..range.end.$idx.clone()),)+) 

            }
        }

        prop_rand_tuple_impls!($($idx $tail),+);

    }
}

prop_rand_tuple_impls!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L, 12 M, 13 N, 14 O, 15 P,);
*/

/* Conflicts with the PropRand for T implemention since PartialOrd COULD be implemented on types
   from bevy.
 
macro_rules! prop_rand_vec_impl {
    ($vec_type:tt, $dim_total:literal, $($dim:ident,)+) => {
        impl PropRand for $vec_type {
            fn gen<R: RngCore + ?Sized>(rng: &mut R) -> Self {
                array![_ => $vec_type::gen(rng); N].into()
            }

            fn gen_range<R: RngCore + ?Sized>(rng: &mut R, range: PropRange<$vec_type>) -> Self {
                PropArray<$vec_type, _>::gen_range(rng, PropRange<$vec_type, _>::from([$(range.start.$dim,)+]..[$(range.end.$dim,)+]))
            }
        }
    }
}

prop_rand_vec_impl!(Vec2, 2, x, y,);

*/
