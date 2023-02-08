#[cfg(target_os = "zkvm")]
pub mod env {
    pub use risc0_zkvm::guest::env::{commit, read};
}
