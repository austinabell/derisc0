use quote::quote;
use syn::{ItemFn, parse_quote};

#[test]
fn basic_codegen() {
    let method: ItemFn = parse_quote! {
        fn some_method(basic: u32, reference: &str, (a, b): (u32, String)) -> u32 {
            println!("{reference} {b}");
            basic + a
        }
    };
	let r0_read = quote!(risc0_zkvm::guest::env::read());
    let output = super::entry::function(method.clone().into());
	// TODO update test to include return codegen
    let expected = quote!(
        #[cfg(target_os = "zkvm")]
        #[no_mangle]
        fn __main() {
            let basic = #r0_read;
            let reference = #r0_read;
            let __arg2 = #r0_read;
            some_method(basic, reference, __arg2);
        }
        #method
    );
    assert_eq!(expected.to_string(), output.to_string());
}
