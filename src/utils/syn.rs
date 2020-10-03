use proc_macro::TokenStream;
use std::vec::IntoIter;

use proc_macro2::Ident;
use syn::{
    parse, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields, Type,
};

pub(crate) fn parse_derive_input(input: TokenStream) -> DeriveInput {
    parse(input).unwrap()
}

pub(crate) fn named_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("This struct does not contain named fields!"),
    }
}

#[allow(dead_code)]
pub(crate) fn all_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => &fields.unnamed,
        _ => panic!("This struct does not contain any fields!"),
    }
}
