use proc_macro::TokenStream;

use quote::{format_ident, quote};

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn getter_mut(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);

    let getters_mut = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let fn_type = field.ty.clone();

        let fn_getter_mut_name = format_ident!("get_{}_mut", field_name);

        quote! {
             pub fn #fn_getter_mut_name(&mut self) -> &mut #fn_type {
                 &mut self.#field_name
             }
        }
    });

    TokenStream::from(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(
                #getters_mut
            )*
        }
    })
}
