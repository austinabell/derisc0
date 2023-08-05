use quote::quote;
use syn::{parse_quote, ItemFn};

#[test]
fn basic_codegen() {
    let method: ItemFn = parse_quote! {
        fn some_method(basic: u32, reference: &str, (a, b): (u32, String)) -> u32 {
            println!("{reference} {b}");
            basic + a
        }
    };
    let output = super::entry::function(method.clone());
    let expected = quote!(
        // Include generated main in a module so we don't conflict
        // with any other definitions of "main" in this file.
        // NOTE: This framework is a modified version of risc0_zkvm::entry!(..);
        mod zkvm_generated_main {
            #[cfg(target_os = "zkvm")]
            #[no_mangle]
            fn main() {
                derisc0::EntryFn::call(super::some_method);
            }
        }
        #method
    );
    assert_eq!(expected.to_string(), output.to_string());
}
