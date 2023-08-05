# derisc0

The purpose of this crate is to create an ergonomic interface and codegen to abstract lower-level interactions with the [risc0](https://github.com/risc0/risc0) VM.

In a very PoC and experimental state. Do not rely on these APIs being stable or maintained.

This API matches a similar pattern to [axum](https://docs.rs/axum/latest/axum)/[tower](https://docs.rs/tower/latest/tower/) where any compatible function that implements `EntryFn` (akin to [`axum::handler::Handler`](https://docs.rs/axum/latest/axum/handler/trait.Handler.html)) can be used as an entry point for a risc0 program. This library uses `derisc0::FromParameter` (akin to [`axum::extract::FromRequestParts`](https://docs.rs/axum/latest/axum/extract/trait.FromRequestParts.html)) for parameters and `derisc0::IntoResult` (akin to [`axum::response::IntoResponse`](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)) for result values.

Currently, parameters without an explicit wrapper are expected to be deserialized from input by default. Return values can be wrapped in `derisc0::Binary` to indicate that the value will be serialized with a binary protocol and committed. Result handling works by panicking with any error type that implements `derisc0::IntoError`.

```rs
use derisc0::Binary;

derisc0::entry!(some_method);

fn some_method(a: u32, b: &str) -> Binary(u32) {
    println!("{b}");
    Binary(a)
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
    env::commit(&a);
}
```

Future plans:
- [x] Result handling through return for ergonomic error handling
- [ ] Include other abstractions for the guest APIs for the zkvm
- [ ] Remove need for domain specific file structure and semantics to generate bytecode and interfaces
- [ ] Create typesafe codegen for interacting with program externally
  - [ ] Testing framework around typesafe interface (fuzzing?)
