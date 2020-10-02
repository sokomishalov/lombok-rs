use proc_macro::TokenStream as TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

pub(crate) fn no_args_constructor(input: DeriveInput) -> TokenStream {
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
    let structure_params = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            quote! {
                #field_name: ::core::default::Default::default(),
            }
        });

    TokenStream2::from(quote! {
        pub fn new() -> Self {
            Self {
                #(
                    #structure_params
                )*
            }
        }
    })
}