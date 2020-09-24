use proc_macro::TokenStream as TokenStream;

use Data::Struct;
use Fields::Named;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data,
    DataStruct,
    DeriveInput,
    Fields,
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
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => return TokenStream2::new(),
    };
    let field_names = fields.iter().map(|field| &field.ident);
    let field_types = fields.iter().map(|field| &field.ty);

    let struct_name = &input.ident;

    TokenStream2::from(quote! {
        #(
            fn #field_names(&self) -> #field_types {
                &self.#field_names
            }
        )*
    })
}