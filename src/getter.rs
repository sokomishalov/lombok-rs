use proc_macro::{TokenStream as TokenStream, TokenTree};

use Data::Struct;
use Fields::Named;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data,
    DataStruct,
    DeriveInput,
    export::ToTokens,
    Fields,
    spanned::Spanned,
};
use syn::export::TokenStreamExt;

pub(crate) fn getter(input: DeriveInput) -> TokenStream {
    let name = input.ident.clone();
    let body = generate_body(input);

    TokenStream::from(quote! {
        impl #name {
            #body
        }
    })
}

fn generate_body(input: DeriveInput) -> TokenStream2 {
    let mut body = TokenStream2::new();
    match input.data {
        Struct(DataStruct { fields: Named(fields), .. }) => {
            for name in fields.named {
                println!("{}", name.ident.unwrap());
            };
        }
        _ => {}
    };
    body
}