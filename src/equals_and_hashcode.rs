use proc_macro::TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn equals_and_hashcode(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();

    let eq_body = generate_eq_body(&derive_input);
    let partial_eq_body = generate_partial_eq_body(&derive_input);
    let hash_body = generate_hash_body(&derive_input);

    TokenStream::from(quote! {
        impl #impl_generics ::core::cmp::Eq for #name #ty_generics #where_clause {
            #eq_body
        }

        impl #impl_generics ::core::marker::StructuralPartialEq for #name #ty_generics #where_clause {

        }

        impl #impl_generics ::core::cmp::PartialEq for #name #ty_generics #where_clause {
            #partial_eq_body
        }

        impl #impl_generics ::core::hash::Hash for #name #ty_generics #where_clause {
            #hash_body
        }
    })
}

fn generate_eq_body(input: &DeriveInput) -> TokenStream2 {
    let fields = named_fields(&input);

    let param_assertions = fields.iter().map(|field| {
        let field_type = field.ty.clone();

        quote! {
            let _: ::core::cmp::AssertParamIsEq<#field_type>;
        }
    });

    TokenStream2::from(quote! {
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                #(
                    #param_assertions
                )*
            }
        }
    })
}

fn generate_partial_eq_body(input: &DeriveInput) -> TokenStream2 {
    let fields = named_fields(&input);
    let name = &input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    TokenStream2::from(quote! {
        fn eq(&self, other: &#name#ty_generics) -> bool {
            true
        }

        fn ne(&self, other: &#name#ty_generics) -> bool {
            true
        }
    })
}

fn generate_hash_body(input: &DeriveInput) -> TokenStream2 {
    let fields = named_fields(&input);
    let name = &input.ident.clone();

    let struct_refs = fields.iter().enumerate().map(|(i, field)| {
        let field_name = field.ident.clone().unwrap();
        let reference = format_ident!("__self_0_{}", i.to_string());

        quote! {
            #field_name: ref #reference,
        }
    });

    let hashes = fields.iter().enumerate().map(|(i, field)| {
        let reference = format_ident!("__self_0_{}", i.to_string());

        quote! {
             ::core::hash::Hash::hash(&(*#reference), state);
        }
    });

    TokenStream2::from(quote! {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                #name {
                    #(
                        #struct_refs
                    )*
                } => {
                    #(
                        #hashes
                    )*
                }
            }
        }
    })
}
