use proc_macro::TokenStream as TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
};
use syn::DeriveInput;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn setter(input: TokenStream) -> TokenStream {
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
    let fields = named_fields(&input);

    let getters = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            let fn_type = field.ty.clone();
            let fn_setter_name = format_ident!("set_{}", field_name);

            quote! {
                pub fn #fn_setter_name(&mut self, #field_name: #fn_type) {
                    self.#field_name = #field_name
                }
            }
        });

    TokenStream2::from(quote! {
        #(
            #getters
        )*
    })
}