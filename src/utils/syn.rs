use proc_macro::TokenStream;

use syn::{
    Data,
    DataStruct,
    DeriveInput,
    Field,
    Fields,
    parse,
    punctuated::Punctuated,
    token::Comma,
};

pub(crate) fn parse_derive_input(input: TokenStream) -> DeriveInput {
    parse(input).unwrap()
}

pub(crate) fn named_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("There is no named fields "),
    }
}