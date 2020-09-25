extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};

use all_args_constructor::all_args_constructor;
use getter::getter;
use getter_mut::getter_mut;
use setter::setter;

mod getter;
mod getter_mut;
mod setter;
mod all_args_constructor;

#[proc_macro_derive(Getter)]
pub fn derive_getter(item: TokenStream) -> TokenStream {
    getter(parse_macro_input!(item as DeriveInput))
}

#[proc_macro_derive(GetterMut)]
pub fn derive_getter_mut(item: TokenStream) -> TokenStream {
    getter_mut(parse_macro_input!(item as DeriveInput))
}

#[proc_macro_derive(Setter)]
pub fn derive_setter(item: TokenStream) -> TokenStream {
    setter(parse_macro_input!(item as DeriveInput))
}

#[proc_macro_derive(AllArgsConstructor)]
pub fn derive_all_args_constructor(item: TokenStream) -> TokenStream {
    all_args_constructor(parse_macro_input!(item as DeriveInput))
}