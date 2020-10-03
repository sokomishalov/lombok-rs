use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn no_args_constructor(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let body = generate_body(&derive_input);

    TokenStream::from(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #body
        }
    })
}

fn generate_body(input: &DeriveInput) -> TokenStream2 {
    let fields = named_fields(&input);

    let structure_params = fields.iter().map(|field| {
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
