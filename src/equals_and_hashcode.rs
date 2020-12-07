use proc_macro::TokenStream;

use quote::{format_ident, quote};

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn equals_and_hashcode(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);

    let param_assertions = fields.iter().map(|field| {
        let field_type = field.ty.clone();

        quote! {
            let _: ::core::cmp::AssertParamIsEq<#field_type>;
        }
    });

    let struct_refs_first = fields.iter().enumerate().map(|(i, field)| {
        let field_name = field.ident.clone().unwrap();
        let reference = format_ident!("__self_1_{}", i.to_string());

        quote! {
            #field_name: ref #reference,
        }
    });
    let struct_refs_second = fields.iter().enumerate().map(|(i, field)| {
        let field_name = field.ident.clone().unwrap();
        let reference = format_ident!("__self_0_{}", i.to_string());

        quote! {
            #field_name: ref #reference,
        }
    });

    let struct_refs_second_eq = struct_refs_second.clone();
    let struct_refs_first_eq = struct_refs_first.clone();
    let struct_refs_second_ne = struct_refs_second.clone();
    let struct_refs_first_ne = struct_refs_first.clone();

    let equalities = fields.iter().enumerate().map(|(i, _)| {
        let reference_first = format_ident!("__self_0_{}", i.to_string());
        let reference_second = format_ident!("__self_1_{}", i.to_string());

        quote! {
            (*#reference_first) == (*#reference_second) &&
        }
    });
    let non_equalities = fields.iter().enumerate().map(|(i, _)| {
        let reference_first = format_ident!("__self_0_{}", i.to_string());
        let reference_second = format_ident!("__self_1_{}", i.to_string());

        quote! {
            (*#reference_first) != (*#reference_second) ||
        }
    });

    let hashes = fields.iter().enumerate().map(|(i, _)| {
        let reference = format_ident!("__self_0_{}", i.to_string());

        quote! {
             ::core::hash::Hash::hash(&(*#reference), state);
        }
    });

    TokenStream::from(quote! {
        impl #impl_generics ::core::cmp::Eq for #name #ty_generics #where_clause {
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    #(
                        #param_assertions
                    )*
                }
            }
        }

        impl #impl_generics ::core::marker::StructuralPartialEq for #name #ty_generics #where_clause {

        }

        impl #impl_generics ::core::cmp::PartialEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &TestNamedStructure#ty_generics) -> bool {
                match *other {
                    #name {
                        #(
                            #struct_refs_second_eq
                        )*
                    } => match *self {
                        #name {
                            #(
                                #struct_refs_first_eq
                            )*
                        } => {
                            #(
                                #equalities
                            )* true
                        }
                    },
                }
            }

            fn ne(&self, other: &TestNamedStructure#ty_generics) -> bool {
                match *other {
                    #name {
                        #(
                            #struct_refs_second_ne
                        )*
                    } => match *self {
                        #name {
                            #(
                                #struct_refs_first_ne
                            )*
                        } => {
                            #(
                                #non_equalities
                            )* false
                        }
                    },
                }
            }
        }

        impl #impl_generics ::core::hash::Hash for #name #ty_generics #where_clause {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                match *self {
                    #name {
                        #(
                            #struct_refs_second
                        )*
                    } => {
                        #(
                            #hashes
                        )*
                    }
                }
            }
        }
    })
}
