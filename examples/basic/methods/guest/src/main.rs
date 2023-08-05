#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use derisc0::Binary;

derisc0::entry!(some_method);

fn some_method(a: u32, b: u32) -> Result<Binary<u32>, &'static str> {
    a.checked_mul(b).map(Binary).ok_or("integer overflow")
}
