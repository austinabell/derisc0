use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, ItemFn};

pub(super) fn function(input: ItemFn) -> TokenStream {
    let fn_name = input.sig.ident.clone();

    let result = quote_spanned! {input.sig.span()=>
        // Include generated main in a module so we don't conflict
        // with any other definitions of "main" in this file.
        // NOTE: This framework is a modified version of risc0_zkvm::entry!(..);
        mod zkvm_generated_main {
            #[cfg(target_os = "zkvm")]
            #[no_mangle]
            fn main() {
                derisc0::EntryFn::call(super::#fn_name);
            }
        }
        #input
    };

    result
}
