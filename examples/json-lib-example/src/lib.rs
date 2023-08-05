use derisc0::{FromParameter, IntoResponse};
use risc0_zkvm::guest::env;
use serde::{de::DeserializeOwned, Serialize};

/// Wrapper type to handle JSON (de)serialization of risc0 input and commitments.
/// 
/// ```no_run
/// use json_lib_example::Json;
/// 
/// derisc0::entry!(some_function);
/// 
/// fn some_function(Json(a): Json<u32>, Json(b): Json<u32>) -> Result<Json<u32>, &'static str> {
///     Ok(Json(a.checked_mul(b).ok_or("integer overflow")?))
/// }
/// # fn main() {
/// # }
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Json<T>(pub T);

impl<T> From<T> for Json<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> FromParameter for Json<T>
where
    T: DeserializeOwned,
{
    fn from_parameter() -> Self {
        Self(serde_json::from_reader(env::stdin()).unwrap())    
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn handle_response(self) {
        serde_json::to_writer(env::stdout(), &self.0).unwrap();
    }
}

