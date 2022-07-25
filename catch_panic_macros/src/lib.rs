use std::ops::Deref;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, AttributeArgs, Expr, FnArg, ItemFn, Lit, Meta, MetaNameValue, NestedMeta, Pat, Path};

const ONLY_FUNCTIONS_MSG: &str = "#[catch_panic] can only be applied to functions";
const FIRST_ARG_MSG: &str = "#[catch_panic] requires that this function has at least one argument of type jni::JNIEnv";
const INVALID_NAME_MSG: &str = "Invalid function parameter name";

/// Catches a panic and rethrows it as a Java exception.
/// See the [crate-level documentation](../catch_panic/index.html) for more information.
#[proc_macro_attribute]
pub fn catch_panic(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse macro arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut default_value = None;
    let mut handler = None;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            path,
            lit: Lit::Str(str),
            ..
        })) = arg
        {
            match path.get_ident() {
                Some(id) if id == "default" => match syn::parse_str::<Expr>(&str.value()) {
                    Ok(expr) => default_value = Some(expr),
                    Err(err) => return TokenStream::from(err.to_compile_error()),
                },
                Some(id) if id == "handler" => match syn::parse_str::<Path>(&str.value()) {
                    Ok(expr) => handler = Some(expr),
                    Err(err) => return TokenStream::from(err.to_compile_error()),
                },
                _ => {}
            }
        }
    }

    let default_value = match default_value {
        Some(val) => val.to_token_stream(),
        None => quote! { ::std::default::Default::default() },
    };
    let handler = match handler {
        Some(val) => val.to_token_stream(),
        None => quote! { ::catch_panic::handler::default_handler },
    };

    // parse and validate function
    let item = proc_macro2::TokenStream::from(item);
    let item_span = item.span();
    let (attrs, vis, sig, block) = match syn::parse2::<ItemFn>(item) {
        Ok(ItemFn { attrs, vis, sig, block }) => (attrs, vis, sig, block),
        Err(_) => {
            return TokenStream::from(quote_spanned! {item_span=>
                compile_error!(#ONLY_FUNCTIONS_MSG);
            })
        }
    };
    if sig.inputs.is_empty() {
        return TokenStream::from(quote_spanned! {sig.span()=>
            compile_error!(#FIRST_ARG_MSG);
        });
    }
    let first_arg_name = match sig.inputs.first().unwrap() {
        FnArg::Receiver(receiver) => {
            return TokenStream::from(quote_spanned! {receiver.span()=>
                compile_error!(#FIRST_ARG_MSG);
            })
        }
        FnArg::Typed(pat_type) => match pat_type.pat.deref() {
            Pat::Ident(pat) => pat.ident.clone(),
            _ => {
                return TokenStream::from(quote_spanned! {pat_type.span()=>
                    compile_error!(#INVALID_NAME_MSG);
                })
            }
        },
    };

    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            ::catch_panic::handler::__catch_panic(#first_arg_name, #default_value, #handler, move || {
                #block
            })
        }
    })
}
