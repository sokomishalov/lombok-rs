use proc_macro::TokenStream as TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

use crate::utils::syn::parse_derive_input;

pub(crate) fn no_args_constructor(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = derive_input.ident.clone();
    let body = generate_body(derive_input);

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