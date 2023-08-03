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

	// -- Exec
	let field_names = Todo::field_names();

	// -- Check
	assert_eq!(field_names, &["id", "description", "name"]);

	// -- Exec & Check
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

#[test]
pub fn test_macro_all_fields() -> Result<()> {
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

	// -- Exec
	let todo = Todo {
		id: 123,
		desc: Some(fx_desc.to_string()),
		name: "name 01".to_string(),
		something_else: "something 01".to_string(),
	};
	let fields = todo.all_fields();

	// -- Check
	assert_eq!("description", &fields[1].name);
	let val = format!("{:?}", &fields[1].value);
	assert_eq!(r#"Some("desc 01")"#, val);

	Ok(())
}

// Note: Just a compile check.
#[allow(unused)]
#[test]
pub fn test_custom_type_and_enum() -> Result<()> {
	use sqlb::bindable;

	#[derive(sqlb::Fields, Default, Clone)]
	pub struct SubscriptionPatch {
		pub client_id: Option<i32>,
		pub my_val: Option<MyEnum>,
	}

	#[derive(sqlx::Type, Debug, Clone)]
	#[sqlx(type_name = "my_enum")] // must be defined in pg as a enum type.
	pub enum MyEnum {
		One,
		Two,
		TooBig,
	}
	sqlb::bindable!(MyEnum);

	#[derive(Debug, Clone, Copy, sqlx::Type)]
	#[sqlx(transparent)]
	pub struct OffsetDateTime(pub time::OffsetDateTime);
	pub struct ClientName(String);

	Ok(())
}

// Note: Just a compile check.
#[test]
pub fn test_bindable_generic() -> Result<()> {
	#[derive(Debug, Clone, sqlx::Type)]
	#[sqlx(transparent)]
	pub struct ClientRef<T>(T);

	sqlb::bindable!(ClientRef<String>);

	Ok(())
}
