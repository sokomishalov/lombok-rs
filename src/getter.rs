use proc_macro::TokenStream;

use quote::{format_ident, quote, ToTokens};
use syn::Type;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn getter(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);

    let getters = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;
        let return_type = match &field.ty {
            Type::Reference(ref_type) => ref_type.to_token_stream(),
            _ => quote! { &#field_type },
        };
        let getter_name = format_ident!("get_{}", field_name);

        quote! {
            pub fn #getter_name(&self) -> #return_type {
                &self.#field_name
            }
        }
    });

    TokenStream::from(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(
                #getters
            )*
        }
    })
}
