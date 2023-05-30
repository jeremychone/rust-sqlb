use anyhow::Result;
use sqlb::HasFields;
use sqlb_macros::Fields;

#[test]
pub fn test_macro_field_names() -> Result<()> {
	// -- Setup & Fixtures
	#[allow(unused)]
	#[derive(Debug, Fields)]
	struct Todo {
		id: i64,

		#[field(name = "description")]
		desc: Option<String>,
		name: String,

		#[field(skip)]
		something_else: String,
	}
	let fx_desc = "desc 01";

	let field_names = Todo::field_names();
	assert_eq!(field_names, &["id", "description", "name"]);

	let todo = Todo {
		id: 123,
		desc: Some(fx_desc.to_string()),
		name: "name 01".to_string(),
		something_else: "something 01".to_string(),
	};
	let fields = todo.all_fields();
	assert_eq!("description", &fields[1].name);
	let val = format!("{:?}", &fields[1].value);
	assert_eq!(r#"Some("desc 01")"#, val);

	Ok(())
}
