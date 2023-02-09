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
    let r0_read = quote!(derisc0::__private::env::read());
    let output = super::entry::function(method.clone());
    let expected = quote!(
        #[cfg(target_os = "zkvm")]
        #[no_mangle]
        fn __main() {
            let basic = #r0_read;
            let reference = #r0_read;
            let __arg2 = #r0_read;
            let __result = some_method(basic, reference, __arg2);
            derisc0::__private::env::commit(&__result);
        }
        #method
    );
    assert_eq!(expected.to_string(), output.to_string());
}
