use crate::{FromParameter, IntoResponse};
use serde::{de::DeserializeOwned, Serialize};

/// Type indicates a parameter or return value that will be (de)serialized by risc0
/// binary serialization protocol and read from input or committed to the journal respectively.
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Binary<T>(pub T);

impl<T> From<T> for Binary<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> FromParameter for Binary<T>
where
    T: DeserializeOwned,
{
    fn from_parameter() -> Self {
        Self(risc0_zkvm::guest::env::read())
    }
}

impl<T> IntoResponse for Binary<T>
where
    T: Serialize,
{
    fn handle_response(self) {
        risc0_zkvm::guest::env::commit(&self.0);
    }
}
