use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{spanned::Spanned, FnArg, ItemFn, Pat, PatIdent, Receiver, ReturnType};

fn positional_arg(i: usize, pat: &Pat) -> Ident {
    let span: Span = syn::spanned::Spanned::span(pat);
    #[cfg(not(no_span_mixed_site))]
    let span = span.resolved_at(Span::mixed_site());
    format_ident!("__arg{}", i, span = span)
}

pub(super) fn function(input: ItemFn) -> TokenStream {
    let r0_env = quote!(derisc0::__private::env);
    let r0_read = quote!(#r0_env::read());
    // TODO abstract this logic
    let decls = input
        .sig
        .inputs
        .iter()
        .enumerate()
        .map(|(i, arg)| match arg {
            FnArg::Receiver(Receiver {
                ..
            }) => {
                // TODO: see what borks first, or implement if support for impls
                unreachable!("self parameter cannot be parsed in a pure function");
            }
            FnArg::Typed(arg) => {
                // If there is a #[cfg(...)] attribute that selectively enables
                // the parameter, forward it to the variable.
                //
                // This is currently not applied to the `self` parameter.
                let attrs = arg.attrs.iter().filter(|attr| attr.path.is_ident("cfg"));

                if let Pat::Ident(PatIdent {
                    ident, mutability, ..
                }) = &*arg.pat
                {
                    if ident == "self" {
                        unreachable!("self parameter cannot be parsed in a pure function");
                    } else {
                        (ident.clone(), quote! {
                            #(#attrs)*
                            let #mutability #ident = #r0_read;
                        })
                    }
                } else {
                    let pat = &arg.pat;
                    let ident = positional_arg(i, pat);
                    (ident.clone(), quote! {
                        #(#attrs)*
                        let #ident = #r0_read;
                    })
                }
            }
        })
        .collect::<Vec<_>>();

    let args: Vec<_> = decls.iter().map(|(ident, _)| ident).collect();
    let read_decls: Vec<_> = decls.iter().map(|(_, decl)| decl).collect();
    let fn_name = input.sig.ident.clone();

	let mut invocation = quote!(#fn_name(#(#args),*););
	if let ReturnType::Type(_, _) = input.sig.output {
		// TODO decide if explicit `-> ()` should be ignored
		invocation = quote! {
			let __result = #invocation
			#r0_env::commit(&__result);
		}
	}

    // TODO handle result to commit the data (and handle result patterns)
    let result = quote_spanned! {input.sig.span()=>
        #[cfg(target_os = "zkvm")]
        #[no_mangle]
        fn __main() {
            #(#read_decls)*
            #invocation
        }
        #input
    };

    result
}
