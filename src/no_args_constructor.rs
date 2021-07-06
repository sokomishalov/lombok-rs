use proc_macro::TokenStream;

use quote::quote;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn no_args_constructor(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);

    let structure_params = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        quote! {
            #field_name: ::core::default::Default::default(),
        }
    });

    TokenStream::from(quote! {
        impl #impl_generics ::core::default::Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(
                        #structure_params
                    )*
                }
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            /// TODO find out a better way to provide `new` method without breaking `AllArgsConstructor` by overloading
            pub fn new_default() -> Self {
                ::core::default::Default::default()
            }
        }
    })
}
