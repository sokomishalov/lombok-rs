use proc_macro::TokenStream as TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
};
use syn::DeriveInput;

pub(crate) fn builder(input: DeriveInput) -> TokenStream {
    let name = input.ident.clone();
    let builder_name = format_ident!("{}Builder", name);

    TokenStream::from(quote! {
        impl #name {

        }

        pub struct #builder_name {

        }

        impl #builder_name {

        }
    })
}