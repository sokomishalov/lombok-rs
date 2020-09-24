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

pub(crate) fn setter(input: DeriveInput) -> TokenStream {
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
            let field_name_lower = field_name.clone().to_string().to_lowercase();
            let fn_type = field.ty.clone();

            let fn_setter_name = format_ident!("set_{}", field_name_lower);

            quote! {
                pub fn #fn_setter_name(&mut self, #field_name: #fn_type) {
                    self.#field_name = #field_name
                }
            }
        });

    TokenStream2::from(quote! {
        #(
            #getters
        )*
    })
}