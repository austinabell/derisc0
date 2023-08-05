pub trait IntoResponse {
    fn handle_response(self);
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoPanic,
{
    fn handle_response(self) {
        match self {
            Ok(t) => t.handle_response(),
            Err(e) => e.handle_panic(),
        }
    }
}

macro_rules! impl_into_response {
    ( $($ty:ident),* $(,)? ) => {
        #[allow(non_snake_case)]
        impl<$($ty,)*> IntoResponse for ($($ty,)*)
        where
            $( $ty: IntoResponse, )*
        {

            fn handle_response(self) {
                let ($($ty,)*) = self;

                $(
                    $ty.handle_response();
                )*
            }
        }
    }
}

crate::macros::all_the_tuples!(impl_into_response);

pub trait IntoPanic {
    fn handle_panic(self) -> !;
}

impl<'a> IntoPanic for &'a str {
    fn handle_panic(self) -> ! {
        risc0_zkvm::guest::abort(self)
    }
}
