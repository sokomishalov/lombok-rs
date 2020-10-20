use proc_macro::TokenStream;

use quote::{format_ident, quote};

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn setter(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);

    let setters = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty.clone();
        let setter_name = format_ident!("set_{}", field_name);

        quote! {
            pub fn #setter_name(&mut self, #field_name: #field_type) {
                self.#field_name = #field_name
            }
        }
    });

    TokenStream::from(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(
                #setters
            )*
        }
    })
}
