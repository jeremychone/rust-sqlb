mod utils;

use crate::utils::Todo;
use serial_test::serial;
use sqlb::{Field, HasFields, Raw};
use std::error::Error;
use utils::{init_db, util_fetch_all_todos, TodoPatch};

#[serial]
#[tokio::test]
async fn sb_insert_ok_simple() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// -- Fixtures
	let test_title = "test - title 01";
	let patch_data = TodoPatch {
		title: Some(test_title.to_string()),
		desc: None,
	};

	// -- Exec
	// Note: test schema and fully qualified column name.
	let sb = sqlb::insert().table("public.todo").data(patch_data.not_none_fields());
	let sb = sb.returning(&["todo.id", "public.todo.title"]);
	let (_id, title) = sb.fetch_one::<_, (i64, String)>(&db_pool).await?;
	assert_eq!(test_title, title);

	// -- Check
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title, todos[0].title);

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_insert_ok_renamed_field() -> Result<(), Box<dyn Error>> {
	// -- Setup & Fixtures
	let db_pool = init_db().await?;
	let fx_title = "test - title 01";
	let fx_desc = "test - desc 01";

	let patch_data = TodoPatch {
		title: Some(fx_title.to_string()),
		desc: Some(fx_desc.to_string()),
	};

	// -- Exec
	let sb = sqlb::insert().table("todo").data(patch_data.all_fields());
	let sb = sb.returning(&["id", "title", "description"]);
	let (_id, title, desc) = sb.fetch_one::<_, (i64, String, String)>(&db_pool).await?;

	// -- Check
	assert_eq!(fx_title, title);
	assert_eq!(fx_desc, desc);

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_insert_ok_renamed_field_and_get() -> Result<(), Box<dyn Error>> {
	// -- Setup & Fixtures
	let db_pool = init_db().await?;
	let fx_title = "test - title 01";
	let fx_desc = "test - desc 01";

	let patch_data = TodoPatch {
		title: Some(fx_title.to_string()),
		desc: Some(fx_desc.to_string()),
	};

	// -- Exec
	let sb = sqlb::insert().table("todo").data(patch_data.all_fields());
	let sb = sb.returning(&["id"]);
	let (id,): (i64,) = sb.fetch_one(&db_pool).await?;
	let todo: Todo = sqlb::select()
		.table("todo")
		.and_where("id", "=", id)
		.fetch_one(&db_pool)
		.await?;
	assert_eq!(todo.desc, Some(fx_desc.to_string()));

	// -- Check
	// assert_eq!(fx_title, title);

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_insert_ok_raw() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURE
	let test_title = "test - title 02";

	// ACTION
	let fields: Vec<Field> = vec![("ctime", Raw("now()")).into(), ("title", test_title.to_string()).into()];

	let sb = sqlb::insert().table("todo").data(fields);
	let sb = sb.returning(&["id", "title", "ctime"]);
	let (_id, title) = sb.fetch_one::<_, (i64, String)>(&db_pool).await?;
	assert_eq!(test_title, title);

	// CHECK
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title, todos[0].title);

	Ok(())
}
