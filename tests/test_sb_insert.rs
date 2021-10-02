mod utils;

use sqlb::{sqlx_exec, HasFields};
use std::error::Error;
use utils::{init_db, util_fetch_all_todos, TodoPatch};

#[tokio::test]
async fn sb_insert() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title = "test - title 01";
	let patch_data = TodoPatch {
		title: Some(test_title.to_string()),
	};

	// DO insert
	let sb = sqlb::insert("todo").data(patch_data.fields());
	let sb = sb.returning(&["id", "title"]);
	let (_id, title) = sqlx_exec::fetch_as_one::<(i64, String), _, _>(&db_pool, &sb).await?;
	assert_eq!(test_title, title);

	// CHECK with select all
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title, todos[0].title);

	Ok(())
}
