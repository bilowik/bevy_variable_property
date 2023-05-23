use array_macro::array;
use paste::paste;

pub trait VariableProperty<T> {
    fn get_value(&self) -> T;
}


impl<T, U: VariableProperty<T>, const N: usize> VariableProperty<[T; N]> for [U; N] {
    fn get_value(&self) -> [T; N] {
        array![i => self[i].get_value(); N]
    }
}




macro_rules! reverse {
    () => {}; 
    ($self:ident, [$($list:literal,)*], $head:literal, $($tail:literal,)*) => {
       reverse!($self, [$($list,)* $head,], $($tail,)*) 
    };
    ($self:ident, [$($list:literal,)+],) => {
        paste! {(
           $(
               $self.$list.get_value(), 
            )+
        )}
    }
}

macro_rules! variable_property_impls {
    () => {};
    ([], [], []) => {};

    ($self:ident) => {
        variable_property_impls!(
            [T,  T,  T,  T,  T,  T,  T, T, T, T, T, T, T, T, T, T,], 
            [U,  U,  U,  U,  U,  U,  U, U, U, U, U, U, U, U, U, U,],
            [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,]
        );
    };
    (
        [$head:ident],
        [$head2:ident],
        [$head3:literal]
    ) => {
        impl<T, U: VariableProperty<T>> VariableProperty<($head, )> for ($head2, ) {
            fn get_value(&self) -> ($head,) {
                (self.0.get_value(), )
            }
        }
    };

    (
        [$head1:ident, $($tail1:ident,)*], 
        [$head2:ident, $($tail2:ident,)*],
        [$head3:literal, $($tail3:literal,)*]
    ) => {
        impl<T, U: VariableProperty<T>> VariableProperty<($head1, $($tail1),*)> for ($head2, $($tail2),*) {
            fn get_value(&self) -> ($head1, $($tail1),*) {
                //($(expr!(self.$tail3.get_value())),*)
                reverse!(self, [], $head3, $($tail3,)* )

            }
        }

        variable_property_impls!([$($tail1,)*], [$($tail2,)*], [$($tail3,)*]);
    };
}


variable_property_impls!(self);
