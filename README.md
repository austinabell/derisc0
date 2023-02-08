# derisc0

The purpose of this crate is to create an ergonomic interface and codegen to abstract lower-level interactions with the [risc0](https://github.com/risc0/risc0) VM.

In a very PoC and experimental state. Do not rely on these APIs being stable or maintained.

The `entry` annotation can be used on any function with typed parameters and return values which implement `serde::Serialize` and `serde::Deserialize` respectively:

```rs
#[derisc0::entry]
fn some_method(a: u32, b: &str) -> u32 {
    println!("{b}");
    a
}
```

Which is equivalent to:

```rs
use risc0_zkvm::guest::env;

risc0_zkvm::entry!(some_method);

fn some_method() {
    let a: u32 = env::read();
    let b: &str = env::read();
    println!("{b}");
    risc0_zkvm::guest::env::commit(&a);
}
```

Future plans:
- [ ] Result handling through return for ergonomic error handling
- [ ] Include other abstractions for the guest APIs for the zkvm
- [ ] Remove need for domain specific file structure and semantics to generate bytecode and interfaces
- [ ] Create typesafe codegen for interacting with program externally
  - [ ] Testing framework around typesafe interface (fuzzing?)
