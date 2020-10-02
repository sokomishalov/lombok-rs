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

pub(crate) fn getter(input: DeriveInput) -> TokenStream {
    let name = input.ident.clone();
    let body = generate_body(input);

    TokenStream::from(quote! {
        impl #name {
            #body
        }
    })
}

fn generate_body(input: DeriveInput) -> TokenStream2 {
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => return TokenStream2::new(),
    };

    let getters = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            let fn_type = field.ty.clone();

            let fn_getter_name = format_ident!("get_{}", field_name);

            quote! {
                pub fn #fn_getter_name(&self) -> &#fn_type {
                    &self.#field_name
                }
            }
        });

    TokenStream2::from(quote! {
        #(
            #getters
        )*
    })
}