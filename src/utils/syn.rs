use proc_macro::TokenStream;

use syn::{DeriveInput, parse};

pub(crate) fn parse_derive_input(input: TokenStream) -> DeriveInput {
    parse(input).unwrap()
}