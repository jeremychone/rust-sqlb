extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Fields)]
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

	//// Collect properties and the Option<..> properties
	let mut props: Vec<&Option<Ident>> = Vec::new();
	let mut props_opts: Vec<&Option<Ident>> = Vec::new();
	for field in fields.named.iter() {
		// NOTE: By macro limitation, we can do only type name match and it would not support type alias
		//       For now, assume Option is use as is, not even in a fully qualified way.
		//       We can add other variants of Option if proven needed
		let type_name = format!("{}", &field.ty.to_token_stream());
		if type_name.contains("Option ") {
			props_opts.push(&field.ident);
		} else {
			props.push(&field.ident);
		}
	}

	//// Generate each type of .pushes code block
	let ff_pushes = quote! {
		#(
			ff.push((stringify!(#props), self.#props.clone()).into());
		)*
	};

	let ff_opt_pushes = quote! {
		#(
			if let Some(val) = &self.#props_opts {
				ff.push((stringify!(#props_opts), val.clone()).into());
			}
		)*
	};

	//// Compose the final code
	let output = quote! {
		impl sqlb::HasFields for #struct_name {
			fn fields(&self) -> Vec<sqlb::Field> {
				let mut ff: Vec<sqlb::Field> = Vec::new();
				#ff_pushes
				#ff_opt_pushes
				ff
			}
		}
	};

	output.into()
}
