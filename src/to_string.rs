use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn to_string(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let body = generate_body(&derive_input);

    TokenStream::from(quote! {
        impl #impl_generics ToString for #name #ty_generics #where_clause {
            fn to_string(&self) -> String {
                #body
            }
        }
    })
}

fn generate_body(input: &DeriveInput) -> TokenStream2 {
    let fields = named_fields(&input);
    let start = format!("{} ", &input.ident.clone());
    let end = ")";

    let enrichers = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let injector = format!("{} = {}{}", field_name, "{", "}");

        quote! {
            accum.push_str(format!(#injector, &self.#field_name).to_owned());
        }
    });

    TokenStream2::from(quote! {
        let mut accum = #start.to_owned();
        #(
            #enrichers
        )*
        accum.push_str(#end);
        return accum
    })
}
