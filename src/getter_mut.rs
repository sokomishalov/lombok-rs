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

pub(crate) fn getter_mut(input: DeriveInput) -> TokenStream {
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

    let getters_mut = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();
            let field_name_lower = field_name.clone().to_string().to_lowercase();
            let fn_type = field.ty.clone();

            let fn_getter_mut_name = format_ident!("get_{}_mut", field_name_lower);

            quote! {
                 pub fn #fn_getter_mut_name(&mut self) -> &mut #fn_type {
                     &mut self.#field_name
                 }
            }
        });

    TokenStream2::from(quote! {
        #(
            #getters_mut
        )*
    })
}