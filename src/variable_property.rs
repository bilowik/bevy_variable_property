use array_macro::array;
use paste::paste;
use bevy_reflect::TypePath;

pub trait VariableProperty {
    type Output: TypePath;
    fn get_value(&self) -> Self::Output;
}

impl<T: TypePath, U: VariableProperty<Output = T>, const N: usize> VariableProperty for [U; N] {
    type Output = [T; N];

    fn get_value(&self) -> [T; N] {
        array![i => self[i].get_value(); N]
    }
}

macro_rules! reverse_types_output {
    () => {};
    ([$($list:tt,)*], $head:tt, $($tail:tt,)*) => {
       reverse_types_output!([$head, $($list,)*], $($tail,)*)
    };
    ([$($list:tt,)+],) => {
        paste! {(
            $($list::Output,)+
        )}
    };
}

macro_rules! reverse_get_value {
    () => {};
    ($self:ident, [$($list:literal,)*], $head:literal, $($tail:literal,)*) => {
       reverse_get_value!($self, [$head, $($list,)*], $($tail,)*)
    };
    ($self:ident, [$($list:literal,)+],) => {
        paste! {(
            $($self.$list.get_value(),)+
        )}
    };
}

macro_rules! reverse_types {
    () => {};
    ([$($list:expr,)*], $head:expr, $($tail:expr,)*) => {
       reverse_types!([$head, $($list,)*], $($tail,)*)
    };
    ([$($list:expr,)+],) => {
        paste! {(
            $($list,)+
        )}
    };
}

macro_rules! variable_property_impls {
    () => {};
    ($head_idx:literal $head_type:tt, $($idx:literal $type:tt,)*) => {

        impl<$head_type: VariableProperty,$($type: VariableProperty,)*> VariableProperty for reverse_types!([], $head_type, $($type,)*) {
            type Output = reverse_types_output!([], $head_type, $($type,)*);
            fn get_value(&self) -> Self::Output {
                reverse_get_value!(self, [], $head_idx, $($idx,)*)
            }
        }


        variable_property_impls!($($idx $type,)*);
    };
}

variable_property_impls!(11 L,  10 K,  9 J, 8 I, 7 H, 6 G, 5 F, 4 E, 3 D, 2 C, 1 B, 0 A,);
