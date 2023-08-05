pub trait FromParameter {
    fn from_parameter() -> Self;
}

macro_rules! impl_from_parameter {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<$($ty,)*> FromParameter for ($($ty,)*)
        where
            $( $ty: FromParameter, )*
        {
            fn from_parameter() -> Self {
                $(
                    let $ty = $ty::from_parameter();
                )*

				($($ty,)*)
            }
        }
    }
}

crate::macros::all_the_tuples!(impl_from_parameter);
