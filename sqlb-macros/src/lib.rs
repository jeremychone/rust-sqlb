mod derive_fields;
mod utils;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Fields, attributes(field))]
pub fn derive_fields(input: TokenStream) -> TokenStream {
	derive_fields::derive_fields_inner(input)
}
