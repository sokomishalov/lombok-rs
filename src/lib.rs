extern crate proc_macro;

use proc_macro::TokenStream as TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Getter)]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded: TokenStream2 = quote! {
        impl #name {
            // todo
        }
    };

    return TokenStream::from(expanded);
}