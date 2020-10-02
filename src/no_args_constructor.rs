use proc_macro::TokenStream as TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn no_args_constructor(input: DeriveInput) -> TokenStream {
    let name = input.ident.clone();

    TokenStream::from(quote! {
        impl #name {

        }
    })
}