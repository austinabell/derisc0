#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::entry!(some_method);

fn some_method() {
    let a: u32 = env::read();
    let b: u32 = env::read();
    env::commit(&a.checked_mul(b).unwrap());
}
