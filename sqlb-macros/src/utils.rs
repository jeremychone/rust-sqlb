use syn::{Attribute, Field};

pub fn get_attribute<'a>(field: &'a Field, name: &str) -> Option<&'a Attribute> {
	field.attrs.iter().find(|a| a.path().is_ident(name))
}
