pub trait IntoResponse {
    fn handle_response(self);
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoError,
{
    fn handle_response(self) {
        match self {
            Ok(t) => t.handle_response(),
            Err(e) => e.handle_error(),
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

pub trait IntoError {
    fn handle_error(self);
}

// TODO probably don't want this generic impl
impl<'a, T> IntoError for T where T: AsRef<&'a str> {
	fn handle_error(self) {
		risc0_zkvm::guest::abort(self.as_ref());
	}
}