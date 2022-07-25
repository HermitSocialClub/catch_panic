use std::ops::Deref;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, AttributeArgs, Expr, FnArg, ItemFn, Lit, Meta, MetaNameValue, NestedMeta, Pat};

const ONLY_FUNCTIONS_MSG: &str = "#[catch_panic] can only be applied to functions";
const FIRST_ARG_MSG: &str = "#[catch_panic] requires that this function has at least one argument of type jni::JNIEnv";
const INVALID_NAME_MSG: &str = "Invalid function parameter name?";

/// Catches a panic and rethrows it as a Java exception.
#[proc_macro_attribute]
pub fn catch_panic(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let maybe_default_value = args
        .first()
        .map(|x| match x {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(str),
                ..
            })) => {
                if path.is_ident("default") {
                    Some(str)
                } else {
                    None
                }
            }
            _ => None,
        })
        .flatten();
    let maybe_default_value = match maybe_default_value {
        Some(str) => match syn::parse_str::<Expr>(&str.value()) {
            Ok(expr) => Some(expr),
            Err(err) => return TokenStream::from(err.to_compile_error()),
        },
        None => None,
    };

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
    TokenStream::from(match maybe_default_value {
        Some(default) => {
            quote! {
                #(#attrs)*
                #vis #sig {
                    crate::util::__catch_panic(#first_arg_name, #default, move || {
                        #block
                    })
                }
            }
        }
        None => {
            quote! {
                #(#attrs)*
                #vis #sig {
                    crate::util::__catch_panic_default(#first_arg_name, move || {
                        #block
                    })
                }
            }
        }
    })
}
