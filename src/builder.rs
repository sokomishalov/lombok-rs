use proc_macro::TokenStream as TokenStream;

use proc_macro2::TokenStream as TokenStream2;
use quote::{
    format_ident,
    quote,
};
use syn::{
    Data,
    DataStruct,
    DeriveInput,
    Fields,
};

use crate::utils::syn::parse_derive_input;

pub(crate) fn builder(input: TokenStream) -> TokenStream {
    let derive_input = parse_derive_input(input);

    let name = derive_input.ident.clone();
    let builder_name = format_ident!("{}Builder", name);
    let visibility = derive_input.vis.clone();

    let builder_struct = generate_builder_struct(&derive_input);
    let struct_impl = generate_struct_impl(&derive_input);
    let builder_impl = generate_builder_impl(&derive_input);

    TokenStream::from(quote! {
        #visibility struct #builder_name {
            #builder_struct
        }

        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #struct_impl
                }
            }
        }

        impl #builder_name {
           #builder_impl
        }
    })
}

fn generate_builder_struct(input: &DeriveInput) -> TokenStream2 {
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => return TokenStream2::new(),
    };

    let builder_structure_fields = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            let fn_type = field.ty.clone();

            quote! {
                #field_name: #fn_type,
            }
        });


    TokenStream2::from(quote! {
        #(
            #builder_structure_fields
        )*
    })
}

fn generate_builder_impl(input: &DeriveInput) -> TokenStream2 {
    let name = input.ident.clone();

    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => return TokenStream2::new(),
    };

    let builder_methods = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            let field_type = field.ty.clone();

            quote! {
                pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                    self.#field_name = #field_name;
                    self
                }
            }
        });

    let struct_params = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();

            quote! {
                #field_name: self.#field_name,
            }
        });

    TokenStream2::from(quote! {
        #(
            #builder_methods
        )*

        pub fn build(self) -> #name {
             #name {
                 #(
                     #struct_params
                 )*
             }
        }
    })
}

fn generate_struct_impl(input: &DeriveInput) -> TokenStream2 {
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => return TokenStream2::new(),
    };

    let builder_struct_params = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();

            quote! {
                #field_name: ::core::default::Default::default(),
            }
        });

    TokenStream2::from(quote! {
        #(
            #builder_struct_params
        )*
    })
}