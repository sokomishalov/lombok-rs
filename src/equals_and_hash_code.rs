use proc_macro::TokenStream;

use quote::{format_ident, quote};

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn equals_and_hash_code(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let fields = named_fields(&derive_input);

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
        use std::hash::{Hash, Hasher};

        impl #impl_generics #name #ty_generics #where_clause {
            fn equals(&self, other: &#name #ty_generics) -> bool {
                self.eq(&other)
            }

            fn hash_code(&self) -> u64 {
                let mut hasher = ::std::collections::hash_map::DefaultHasher::new();
                &self.hash(&mut hasher);
                hasher.finish()
            }
        }

        impl #impl_generics ::core::cmp::PartialEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &#name #ty_generics) -> bool {
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

            fn ne(&self, other: &#name #ty_generics) -> bool {
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
