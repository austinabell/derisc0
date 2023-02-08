use proc_macro::TokenStream;

#[cfg(test)]
mod tests;

mod entry;

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

/// The `#[entry]` attribute can be used to declare the entry point of a risc0 program.
/// 
/// This attribute will generate codegen to pull the function parameters from risc0 input, and
/// use the return value to commit to the receipt of the execution.
/// 
/// # Example
/// 
/// ```ignore
/// #[derisc0::entry]
/// fn some_method(a: u32, b: &str) -> u32 {
///     println!("{b}");
///     a
/// }
/// ```
/// 
/// Which is equivalent to:
/// 
/// ```ignore
/// risc0_zkvm::entry!(main);
/// 
/// fn some_method() {
///     let a: u32 = risc0_zkvm::guest::env::read();
///     let b: &str = risc0_zkvm::guest::env::read();
///     println!("{b}");
///     risc0_zkvm::guest::env::commit(&a);
/// }
/// ```
#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // TODO allow overriding the crate to use for the risc0 codegen
    // If any of the steps for this macro fail, we still want to expand to an item that is as close
    // to the expected output as possible. This helps out IDEs such that completions and other
    // related features keep working.
    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    entry::function(input).into()
}
