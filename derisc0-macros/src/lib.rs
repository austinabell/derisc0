use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, quote_spanned};
use syn::{spanned::Spanned, FnArg, Pat, PatIdent, Receiver};

fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(TokenStream::from(error.into_compile_error()));
    tokens
}

fn positional_arg(i: usize, pat: &Pat) -> Ident {
    let span: Span = syn::spanned::Spanned::span(pat);
    #[cfg(not(no_span_mixed_site))]
    let span = span.resolved_at(Span::mixed_site());
    format_ident!("__arg{}", i, span = span)
}

#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // If any of the steps for this macro fail, we still want to expand to an item that is as close
    // to the expected output as possible. This helps out IDEs such that completions and other
    // related features keep working.
    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    // TODO might need this to give better spanned error messages
    // // If type mismatch occurs, the current rustc points to the last statement.
    // let (last_stmt_start_span, last_stmt_end_span) = {
    //     let mut last_stmt = input
    //         .block
    //         .stmts
    //         .last()
    //         .map(ToTokens::into_token_stream)
    //         .unwrap_or_default()
    //         .into_iter();
    //     // `Span` on stable Rust has a limitation that only points to the first
    //     // token, not the whole tokens. We can work around this limitation by
    //     // using the first/last span of the tokens like
    //     // `syn::Error::new_spanned` does.
    //     let start = last_stmt.next().map_or_else(Span::call_site, |t| t.span());
    //     let end = last_stmt.last().map_or(start, |t| t.span());
    //     (start, end)
    // };

    // TODO pull this into a private module inside derisc
    let r0_env = quote!(risc0_zkvm::guest::env);
    let r0_read = quote!(#r0_env::read());
    // TODO abstract this logic
    let decls = input
        .sig
        .inputs
        .iter()
        .enumerate()
        .map(|(i, arg)| match arg {
            FnArg::Receiver(Receiver {
                // self_token,
                // mutability,
                ..
            }) => {
                // TODO: see what borks first, or implement if support for impls
                unreachable!("self parameter cannot be parsed in a pure function");
                // let ident = Ident::new("__self", self_token.span);
                // self_span = Some(self_token.span);
                // quote!(let #mutability #ident = #self_token;)
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
                        // self_span = Some(ident.span());
                        // let prefixed = Ident::new("__self", ident.span());
                        // quote!(let #mutability #prefixed = #ident;)
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
                    if let Pat::Wild(_) = **pat {
                        (ident.clone(), quote! {
                            #(#attrs)*
                            let #ident = #r0_read;
                        })
                    } else {
                        (ident.clone(), quote! {
                            #(#attrs)*
                            let #pat = {
                                let #ident = #r0_read;
                                #ident
                            };
                        })
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let args: Vec<_> = decls.iter().map(|(ident, _)| ident).collect();
    let read_decls: Vec<_> = decls.iter().map(|(_, decl)| decl).collect();
    let fn_name = input.sig.ident.clone();

    // TODO handle result to commit the data (and handle result patterns)
    let result = quote_spanned! {input.sig.span() =>
        #[cfg(target_os = "zkvm")]
        #[no_mangle]
        fn __main() {
            #(#read_decls)*
            #fn_name(#(#args),*);
        }
        #input
    };

    result.into()
}
