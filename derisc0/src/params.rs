use serde::de::DeserializeOwned;

pub trait FromParameter {
    fn from_parameter() -> Self;
}

// TODO revisit this before any releases. Probably a bit too magical to assume deserialization
// protocol for inputs.
impl<T> FromParameter for T
where
    T: DeserializeOwned,
{
    fn from_parameter() -> Self {
        risc0_zkvm::guest::env::read()
    }
}

// macro_rules! impl_from_parameter {
//     ( $($ty:ident),* $(,)? ) => {
//         #[allow(non_snake_case)]
//         impl<$($ty,)*> FromParameter for ($($ty,)*)
//         where
//             $( $ty: FromParameter, )*
//         {
//             fn from_parameter() -> Self {
//                 $(
//                     let $ty = $ty::from_parameter();
//                 )*

// 				($($ty,)*)
//             }
//         }
//     }
// }

// crate::macros::all_the_tuples!(impl_from_parameter);
