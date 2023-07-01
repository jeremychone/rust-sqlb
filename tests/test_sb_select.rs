mod utils;

use crate::utils::{util_insert_many_todos, Todo};
use serial_test::serial;
use std::error::Error;
use utils::init_db;

#[serial]
#[tokio::test]
async fn sb_select_ok_simple() -> Result<(), Box<dyn Error>> {
	// -- Setup & Fixtures
	let db_pool = init_db().await?;
	let fx_title_prefix = "test_sb_select_ok_simple";
	let fx_ids = util_insert_many_todos(&db_pool, fx_title_prefix, 5).await?;

	// -- Exec
	let todos: Vec<Todo> = sqlb::select().table("todo").fetch_all(&db_pool).await?;

	// -- Check
	// Todo: Needs to do a more complete check
	let todos: Vec<Todo> = todos.into_iter().filter(|t| t.title.starts_with(fx_title_prefix)).collect();
	assert_eq!(todos.len(), 5, "number of todos");

	// -- Clean
	for id in fx_ids {
		sqlb::delete().table("todo").and_where_eq("id", id).exec(&db_pool).await?;
	}

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_select_ok_count() -> Result<(), Box<dyn Error>> {
	// -- Setup & Fixtures
	let db_pool = init_db().await?;
	let fx_title_prefix = "test_sb_select_ok_simple";
	let fx_ids = util_insert_many_todos(&db_pool, fx_title_prefix, 5).await?;

	// -- Exec
	let (count,): (i64,) = sqlb::select().table("todo").columns(&["count(*)"]).fetch_one(&db_pool).await?;

	// -- Check
	assert_eq!(count, 5, "number of todos");

	// -- Clean
	for id in fx_ids {
		sqlb::delete().table("todo").and_where_eq("id", id).exec(&db_pool).await?;
	}

	Ok(())
}
