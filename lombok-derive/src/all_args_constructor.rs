use proc_macro::TokenStream;

use quote::quote;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn all_args_constructor(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);

    let constructor_fields = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let fn_type = field.ty.clone();

        quote! {
            #field_name: #fn_type,
        }
    });

    let structure_params = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        quote! {
            #field_name,
        }
    });

    TokenStream::from(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn new(#(#constructor_fields)*) -> Self {
                Self {
                    #(
                        #structure_params
                    )*
                }
            }
        }
    })
}
