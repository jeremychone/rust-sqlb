use crate::utils::Todo;
use anyhow::Result;
use sqlb::HasFields;

mod utils;

#[test]
fn test_macro_field_names() -> Result<()> {
	// -- Fixtures
	let fx_names = vec!["id", "title", "description"];

	// -- Exec
	let names = Todo::field_names();

	// -- Check
	assert_eq!(names, fx_names);

	Ok(())
}

#[test]
fn test_macro_fields_all() -> Result<()> {
	// -- Fixtures
	let fx_title = "title 01";
	let todo = Todo {
		id: 123,
		title: fx_title.to_string(),
		desc: Some("description 01".to_string()),

		other: None,
	};

	// -- Exec
	let fields = todo.all_fields();

	// -- Check
	let mut fields = fields.into_iter();

	// id
	let field = fields.next().unwrap();
	assert_eq!(field.name.to_string(), "id".to_string());
	assert_eq!(field.value_into::<i64>()?, 123);

	// title
	let field = fields.next().unwrap();
	assert_eq!(field.name.to_string(), "title".to_string());
	assert_eq!(field.value_into::<String>()?, fx_title.to_string());

	Ok(())
}
