/// Type indicates a return value that will be serialized by risc0
/// binary serialization protocol and committed to the journal.
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Commit<T>(pub T);

impl<T> From<T> for Commit<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl IntoResponse for Commit<T>
where
    T: Serialize,
{
    fn handle_response(self) {
        risc0_zkvm::guest::env::commit(self.0);
    }
}
