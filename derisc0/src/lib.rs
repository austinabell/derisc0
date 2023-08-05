#![deny(dead_code, unused_imports, unused_mut)]
#![cfg_attr(not(feature = "std"), no_std)]

mod binary;
mod entry;
mod params;
mod response;

pub use binary::Binary;
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
