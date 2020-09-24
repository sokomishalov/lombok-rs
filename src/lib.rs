extern crate proc_macro;

use proc_macro::TokenStream as TokenStream;

use syn::{DeriveInput, parse_macro_input};

use getter::getter;
use getter_mut::getter_mut;

mod getter;
mod getter_mut;

#[proc_macro_derive(Getter)]
pub fn derive_getter(item: TokenStream) -> TokenStream {
    getter(parse_macro_input!(item as DeriveInput))
}

#[proc_macro_derive(GetterMut)]
pub fn derive_getter_mut(item: TokenStream) -> TokenStream {
    getter_mut(parse_macro_input!(item as DeriveInput))
}