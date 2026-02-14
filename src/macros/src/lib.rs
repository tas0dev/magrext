use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// それがmodの関数であることを示す
#[proc_macro_attribute]
pub fn shiny_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(item as ItemFn);
    if func.sig.abi.is_none() {
        func.sig.abi = Some(syn::parse_quote!(extern "C"));
    }
    func.attrs.push(syn::parse_quote!(#[no_mangle]));
    TokenStream::from(quote!(#func))
}
