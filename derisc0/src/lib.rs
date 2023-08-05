#![deny(dead_code, unused_imports, unused_mut)]
#![no_std]

// Used by generated code and doc tests. Not public API.
#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;

pub use derisc0_macros::entry;

mod entry;
mod params;
mod response;

pub use entry::EntryFn;
pub use params::FromParameter;
pub use response::{IntoError, IntoResponse};

#[macro_use]
pub(crate) mod macros;
