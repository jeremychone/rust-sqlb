#![allow(unused)] // For early development.
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Attribute, Expr, Field, FieldsNamed, Lit, LitInt, LitStr, Meta, MetaNameValue, Token};

// region:    --- Prop (i.e., sqlb Field)
pub struct Prop<'a> {
	pub name: String,
	pub is_option: bool,
	pub ident: &'a Option<Ident>,
}

pub fn get_props(fields: &FieldsNamed) -> Vec<Prop> {
	let mut props = Vec::new();

	for field in fields.named.iter() {
		// -- Get the FieldAttr
		let field_attr = get_prop_attr(field);

		// TODO: Need to check better handling.
		let field_attr = field_attr.unwrap();
		if field_attr.skip {
			continue;
		}

		// -- ident
		let ident = &field.ident;

		// -- is_option
		// NOTE: By macro limitation, we can do only type name match and it would not support type alias
		//       For now, assume Option is used as is or type name contains it.
		//       We can add other variants of Option if proven needed.
		let type_name = format!("{}", &field.ty.to_token_stream());
		let is_option = type_name.contains("Option ");

		// -- name
		let name = if let Some(name) = field_attr.name {
			name
		} else {
			ident.as_ref().map(|i| i.to_string()).unwrap()
			// quote! {stringify!(#ident)}
		};

		// -- Add to array.
		props.push(Prop { name, is_option, ident })
	}

	props
}
// endregion: --- Prop (i.e., sqlb Field)

// region:    --- Attribute
pub struct PropAttr {
	pub skip: bool,
	pub name: Option<String>,
}

// #[field(skip, name = "new_name")]
// #[field(name = "new_name")]
pub fn get_prop_attr(field: &Field) -> Result<PropAttr, syn::Error> {
	let attribute = get_attribute(field, "field");

	let mut skip = false;
	let mut name: Option<String> = None;

	if let Some(attribute) = attribute {
		let nested = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

		for meta in nested {
			match meta {
				// #[field(skip)]
				Meta::Path(path) if path.is_ident("skip") => {
					skip = true;
				}

				// #[field(name=value)]
				Meta::NameValue(nv) if nv.path.is_ident("name") => {
					if let Expr::Lit(exp_lit) = nv.value {
						if let Lit::Str(lit_str) = exp_lit.lit {
							name = Some(lit_str.value())
						}
					}
				}

				/* ... */
				_ => {
					return Err(syn::Error::new_spanned(meta, "unrecognized field"));
				}
			}
		}
	}

	Ok(PropAttr { skip, name })
}

fn get_attribute<'a>(field: &'a Field, name: &str) -> Option<&'a Attribute> {
	field.attrs.iter().find(|a| a.path().is_ident(name))
}
// endregion: --- Attribute
