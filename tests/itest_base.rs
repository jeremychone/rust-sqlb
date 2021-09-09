mod utils;

use sqlb::{sqlx_exec, GetFields};
use std::error::Error;
use utils::{init_db, Todo, TodoPatch};

/// Simple and dirty very first test. Should be moved to integrated test.
#[tokio::test]
async fn itest_insert_and_select() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	let test_title = "test - title 01";

	let patch_data = TodoPatch {
		title: Some(test_title.to_string()),
	};

	// test insert with returning
	let sb = sqlb::insert("todo").data(patch_data.fields());
	let sb = sb.returning(&["id", "title"]);
	let (_id, title) = sqlx_exec::fetch_as_one::<(i64, String), _>(&db_pool, &sb).await?;
	assert_eq!(test_title, title);

	// test select
	let sb = sqlb::select("todo").columns(&["id", "title"]).order_by("!id");
	let todos: Vec<Todo> = sqlx_exec::fetch_as_all(&db_pool, &sb).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title, todos[0].title);

	Ok(())
}
