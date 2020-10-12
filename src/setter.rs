use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn setter(input: TokenStream) -> TokenStream {
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

    let setters = fields.iter().map(|field| {
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
            #setters
        )*
    })
}
