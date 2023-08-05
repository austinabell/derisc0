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
pub use response::{IntoPanic, IntoResponse};

#[macro_use]
pub(crate) mod macros;

#[cfg(test)]
mod tests {
    use crate::{EntryFn, FromParameter, IntoResponse};

    fn test_entry(t: TestStruct) -> Result<TestStruct, &'static str> {
        Ok(t)
    }

    struct TestStruct(u8);

    impl FromParameter for TestStruct {
        fn from_parameter() -> Self {
            Self(1)
        }
    }

    impl IntoResponse for TestStruct {
        fn handle_response(mut self) {
            self.0 += 2;
        }
    }

    impl Drop for TestStruct {
        fn drop(&mut self) {
            // Test that the parameter was initialized as 1 and incremented by 2 on response handled
            assert_eq!(self.0, 3);
        }
    }

    #[test]
    fn basic_function_translation() {
        EntryFn::call(test_entry);
    }
}
