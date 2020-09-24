extern crate proc_macro;

use proc_macro::TokenStream as TokenStream;

use syn::{DeriveInput, parse_macro_input};

use getter::getter;

mod getter;

#[proc_macro_derive(Getter)]
pub fn derive_getter(item: TokenStream) -> TokenStream {
    getter(parse_macro_input!(item as DeriveInput))
}