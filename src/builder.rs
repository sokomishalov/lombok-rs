use proc_macro::TokenStream;

use quote::{format_ident, quote};

use crate::utils::syn::{named_fields, parse_derive_input};

pub(crate) fn builder(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = &derive_input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = &derive_input.generics.split_for_impl();
    let visibility = derive_input.vis.clone();
    let builder_name = format_ident!("{}Builder", name);

    let fields = named_fields(&derive_input);

    let builder_struct_fields = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let fn_type = field.ty.clone();

        quote! {
            #field_name: #fn_type,
        }
    });

    let builder_struct_params = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();

        quote! {
            #field_name: ::core::default::Default::default(),
        }
    });

    let builder_methods = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = field.ty.clone();

        quote! {
            pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = #field_name;
                self
            }
        }
    });

    let struct_params = fields.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();

        quote! {
            #field_name: self.#field_name.clone(),
        }
    });

    TokenStream::from(quote! {
        #visibility struct #builder_name #ty_generics #where_clause {
            #(
                #builder_struct_fields
            )*
        }

        impl #impl_generics #name #ty_generics #where_clause {
            pub fn builder() -> #builder_name #ty_generics {
                #builder_name {
                    #(
                        #builder_struct_params
                    )*
                }
            }
        }

        impl #impl_generics #builder_name #ty_generics #where_clause {
           #(
                #builder_methods
           )*

            pub fn build(&self) -> #name #ty_generics {
                 #name {
                     #(
                         #struct_params
                     )*
                 }
            }
        }
    })
}
