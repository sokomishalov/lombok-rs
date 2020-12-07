use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn to_string(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);
    let plain_name = &derive_input.ident.clone().to_string();

    let struct_refs = fields.iter().enumerate().map(|(i, field)| {
        let field_name = field.ident.clone().unwrap();
        let reference = format_ident!("__self_0_{}", i.to_string());

        quote! {
            #field_name: ref #reference,
        }
    });
    let formatter_enrichments = fields.iter().enumerate().map(|(i, field)| {
        let plain_field_name = field.ident.clone().unwrap().to_string();
        let reference = format_ident!("__self_0_{}", i.to_string());

        quote! {
            let _ = debug_trait_builder.field(#plain_field_name, &&(*#reference));
        }
    });
    let debug_body = TokenStream2::from(quote! {
        match *self {
            #name {
                #(
                    #struct_refs
                )*
            } => {
                let mut debug_trait_builder = f.debug_struct(#plain_name);
                #(
                    #formatter_enrichments
                )*
                debug_trait_builder.finish()
            }
        }
    });

    TokenStream::from(quote! {
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                #debug_body
            }
        }

        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                (self as &dyn ::std::fmt::Debug).fmt(f)
            }
        }

        // TODO commented due to error[E0119]
        // impl #impl_generics ::std::string::ToString for #name #ty_generics #where_clause {
        //     fn to_string(&self) -> ::std::string::String {
        //         ::std::format!("{:?}", self)
        //     }
        // }
    })
}
