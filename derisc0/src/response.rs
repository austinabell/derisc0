#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
pub use ::std::vec::Vec;
#[cfg(not(feature = "std"))]
pub use alloc::vec::Vec;

pub trait IntoResponse {
    fn handle_response(self);
}

impl IntoResponse for &[u8] {
    fn handle_response(self) {
        risc0_zkvm::guest::env::write_slice(self);
    }
}

impl IntoResponse for Vec<u8> {
    fn handle_response(self) {
        self.as_slice().handle_response();
    }
}

impl<const N: usize> IntoResponse for [u8; N] {
    fn handle_response(self) {
        self.as_slice().handle_response();
    }
}

impl IntoResponse for &[u32] {
    fn handle_response(self) {
        risc0_zkvm::guest::env::write_slice(self);
    }
}

impl IntoResponse for Vec<u32> {
    fn handle_response(self) {
        self.as_slice().handle_response();
    }
}

impl<const N: usize> IntoResponse for [u32; N] {
    fn handle_response(self) {
        self.as_slice().handle_response();
    }
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
