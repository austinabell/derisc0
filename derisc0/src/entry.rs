use crate::{FromParameter, IntoResponse};

// TODO maybe T generic isn't needed or helpful
pub trait EntryFn<T> {
    fn call(self);
}

macro_rules! impl_entry_fn {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, Res, $($ty,)*> EntryFn<($($ty,)*)> for F
        where
            F: FnOnce($($ty,)*) -> Res,
            Res: IntoResponse,
            $( $ty: FromParameter, )*
        {

            fn call(self) {
				$(
					let $ty = <$ty as FromParameter>::from_parameter();
				)*

				let res = self($($ty,)*);

				IntoResponse::handle_response(res)
            }
        }
    };
}

// Implement for the base function
impl_entry_fn!();

// Implement all tuple combinations
crate::macros::all_the_tuples!(impl_entry_fn);
