mod utils;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Fields, attributes(skip_field))]
pub fn derives_fields(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let struct_name = ast.ident;

	//// get the fields
	let fields = if let syn::Data::Struct(syn::DataStruct {
		fields: syn::Fields::Named(ref fields),
		..
	}) = ast.data
	{
		fields
	} else {
		panic!("Only support Struct")
	};

	// -- Collect properties and the Option<..> properties
	let mut props_all: Vec<&Option<Ident>> = Vec::new();
	let mut props_optional: Vec<&Option<Ident>> = Vec::new();
	let mut props_not_optional: Vec<&Option<Ident>> = Vec::new();
	for field in fields.named.iter() {
		// skip if #[skip_field]
		if utils::get_attribute(field, "skip_field").is_some() {
			continue;
		}

		// NOTE: By macro limitation, we can do only type name match and it would not support type alias
		//       For now, assume Option is used as is or type name contains it.
		//       We can add other variants of Option if proven needed.
		let type_name = format!("{}", &field.ty.to_token_stream());

		props_all.push(&field.ident);

		if type_name.contains("Option ") {
			props_optional.push(&field.ident);
		} else {
			props_not_optional.push(&field.ident);
		}
	}

	// -- Vec push code for the (name, value)
	let ff_pushes = quote! {
		#(
			ff.push((stringify!(#props_not_optional), self.#props_not_optional.clone()).into());
		)*
	};

	let ff_not_none_pushes = quote! {
		#(
			if let Some(val) = &self.#props_optional {
				ff.push((stringify!(#props_optional), val.clone()).into());
			}
		)*
	};

	// -- Compose the final code
	let output = quote! {
		impl sqlb::HasFields for #struct_name {

			fn not_none_fields(&self) -> Vec<sqlb::Field> {
				let mut ff: Vec<sqlb::Field> = Vec::new();
				#ff_pushes
				#ff_not_none_pushes
				ff
			}

			fn all_fields(&self) -> Vec<sqlb::Field> {
				let mut ff: Vec<sqlb::Field> = Vec::new();
				#ff_pushes
				#ff_not_none_pushes
				ff
			}

			fn field_names() -> &'static [&'static str] {
				&[#(
					stringify!(#props_all),
				)*]
			}
		}
	};

	output.into()
}
