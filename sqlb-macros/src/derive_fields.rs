use crate::utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

pub(crate) fn derive_fields_inner(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let struct_name = ast.ident;

	// -- get the fields
	let fields = if let syn::Data::Struct(syn::DataStruct {
		fields: syn::Fields::Named(ref fields),
		..
	}) = ast.data
	{
		fields
	} else {
		panic!("Only support Struct")
	};

	// -- Collect Elements
	let props = utils::get_props(fields);

	let props_all_idents: Vec<&Option<Ident>> = props.iter().map(|p| p.ident).collect();
	let props_all_names: Vec<&String> = props.iter().map(|p| &p.name).collect();

	let props_option_idents: Vec<&Option<Ident>> = props.iter().filter(|p| p.is_option).map(|p| p.ident).collect();
	let props_option_names: Vec<&String> = props.iter().filter(|p| p.is_option).map(|p| &p.name).collect();

	let props_not_option_idents: Vec<&Option<Ident>> = props.iter().filter(|p| !p.is_option).map(|p| p.ident).collect();
	let props_not_option_names: Vec<&String> = props.iter().filter(|p| !p.is_option).map(|p| &p.name).collect();

	// -- Vec push code for the (name, value)
	let ff_all_pushes = quote! {
		#(
			// ff.push((#props_all_names, self.#props_all_idents).into());
			ff.push(
				sqlb::Field {
					// name: sqlb::SIden(#props_all_names).into_iden(),
					name: sea_query::IntoIden::into_iden(sqlb::SIden(#props_all_names)),
					value: self.#props_all_idents.into()
				}
			);
		)*
	};

	let ff_not_option_pushes = quote! {
		#(
			// ff.push((#props_not_option_names, self.#props_not_option_idents).into());
			ff.push(
				sqlb::Field {
					// name: sqlb::SIden(#props_not_option_names).into_iden(),
					name: sea_query::IntoIden::into_iden(sqlb::SIden(#props_not_option_names)),
					value: self.#props_not_option_idents.into()
				}
			);
		)*
	};

	let ff_option_not_none_pushes = quote! {
		#(
			if let Some(val) = self.#props_option_idents {
				// ff.push((#props_option_names, val).into());
				ff.push(
					sqlb::Field {
						name: sea_query::IntoIden::into_iden(sqlb::SIden(#props_option_names)),
						value: val.into()
					}
				);
			}
		)*
	};

	// -- Compose the final code
	let output = quote! {
		impl sqlb::HasFields for #struct_name {

			fn not_none_fields<'a>( self) -> sqlb::Fields {
				let mut ff: Vec<sqlb::Field> = Vec::new();
				#ff_not_option_pushes
				#ff_option_not_none_pushes
				sqlb::Fields::new(ff)
			}

			fn all_fields<'a>( self) -> sqlb::Fields {
				let mut ff: Vec<sqlb::Field> = Vec::new();
				#ff_all_pushes
				sqlb::Fields::new(ff)
			}

			fn field_names() -> &'static [&'static str] {
				&[#(
				#props_all_names,
				)*]
			}

			fn field_idens() -> Vec<sea_query::SeaRc<dyn sea_query::Iden>> {
				vec![#(
				sea_query::IntoIden::into_iden(sqlb::SIden(#props_all_names)),
				)*]
			}
		}
	};

	output.into()
}
