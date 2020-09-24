extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Getter)]
pub fn derive_getter(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}