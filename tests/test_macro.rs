use anyhow::Result;
use sqlb::HasFields;
use sqlb_macros::Fields;

#[test]
pub fn test_macro_field_names() -> Result<()> {
	#[allow(unused)]
	#[derive(Debug, Fields)]
	struct Todo {
		id: i64,

		#[field(name=description)]
		desc: Option<String>,
		name: String,

		#[field(skip)]
		something_else: String,
	}

	let field_names = Todo::field_names();

	assert_eq!(field_names, &["id", "description", "name"]);

	Ok(())
}
