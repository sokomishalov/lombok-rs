use proc_macro::TokenStream as TokenStream;
use std::any::Any;

use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
};
use syn::{Data, DataStruct, DeriveInput, Field, Fields};
use syn::punctuated::Punctuated;
use syn::token::Token;

pub(crate) fn builder(input: DeriveInput) -> TokenStream {
    let name = input.ident.clone();
    let builder_name = format_ident!("{}Builder", name);

    let builder_struct = generate_builder_struct(&input);
    let struct_impl = generate_struct_impl(&input);

    TokenStream::from(quote! {
        struct #builder_name #builder_struct

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name #struct_impl
            }
        }

        impl #builder_name {
           fn build(&self) -> #name {

           }
        }
    })
}

fn generate_builder_struct(input: &DeriveInput) -> TokenStream2 {
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => return TokenStream2::new(),
    };

    let builder_structure_fields = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            let fn_type = field.ty.clone();

            quote! {
                #field_name: #fn_type,
            }
        });

    TokenStream2::from(quote! {
        {#(#builder_structure_fields)*}
    })
}

fn generate_builder_impl(input: DeriveInput) -> TokenStream2 {
    TokenStream2::new()
}

fn generate_struct_impl(input: &DeriveInput) -> TokenStream2 {
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => return TokenStream2::new(),
    };

    let builder_structure_params = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            quote! {
                #field_name: ::core::default::Default::default(),
            }
        });

    TokenStream2::from(quote! {
        {#(#builder_structure_params)*}
    })
}