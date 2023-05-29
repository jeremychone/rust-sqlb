#![allow(unused)] // For early development.
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Attribute, Expr, Field, Lit, LitInt, LitStr, Meta, MetaNameValue, Token};
pub struct FieldAttr {
	pub skip: bool,
	pub name: Option<String>,
}

fn get_attribute<'a>(field: &'a Field, name: &str) -> Option<&'a Attribute> {
	field.attrs.iter().find(|a| a.path().is_ident(name))
}

// #[field(skip, name=new_name)]
// #[field(name=new_name)]
pub fn get_field_attr(field: &Field) -> Result<FieldAttr, syn::Error> {
	let attribute = get_attribute(field, "field");

	let mut skip = false;
	let mut name: Option<String> = None;

	if let Some(attribute) = attribute {
		let nested = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

		// println!("->> meta {:?}", attribute.meta));
		for meta in nested {
			match meta {
				// #[field(skip)]
				Meta::Path(path) if path.is_ident("skip") => {
					skip = true;
				}

				// #[field(name=value)]
				Meta::NameValue(nv) => {
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

	Ok(FieldAttr { skip, name })
}
