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
	// Note: Check schema as well.
	let todos: Vec<Todo> = sqlb::select().table("public.todo").fetch_all(&db_pool).await?;
	let todos: Vec<Todo> = todos.into_iter().filter(|t| t.title.starts_with(fx_title_prefix)).collect();

	// -- Check
	// Todo: Needs to do a more complete check
	assert_eq!(todos.len(), 5, "number of todos");

	// -- Clean
	for id in fx_ids {
		sqlb::delete().table("todo").and_where_eq("id", id).exec(&db_pool).await?;
	}

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_select_ok_limit_offset() -> Result<(), Box<dyn Error>> {
	// -- Setup & Fixtures
	let db_pool = init_db().await?;
	let fx_title_prefix = "sb_select_ok_limit_offset";
	let fx_ids = util_insert_many_todos(&db_pool, fx_title_prefix, 5).await?;

	// -- Exec
	let todos: Vec<Todo> = sqlb::select().table("todo").limit(3).offset(2).fetch_all(&db_pool).await?;
	let todos: Vec<Todo> = todos.into_iter().filter(|t| t.title.starts_with(fx_title_prefix)).collect();

	// -- Check
	assert_eq!(todos.len(), 3, "number of todos");
	let todo_02 = todos.get(0).unwrap();
	assert_eq!(todo_02.title, "sb_select_ok_limit_offset-02");

	// -- Clean
	for id in fx_ids {
		sqlb::delete().table("todo").and_where_eq("id", id).exec(&db_pool).await?;
	}

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_select_ok_limit_only() -> Result<(), Box<dyn Error>> {
	// -- Setup & Fixtures
	let db_pool = init_db().await?;
	let fx_title_prefix = "sb_select_ok_limit_only";
	let fx_ids = util_insert_many_todos(&db_pool, fx_title_prefix, 5).await?;

	// -- Exec
	// Note: Check schema as well and fully qualified column name.
	let todos: Vec<Todo> = sqlb::select()
		.table("public.todo")
		.columns(&["id", "todo.title", "public.todo.description"])
		.limit(3)
		.fetch_all(&db_pool)
		.await?;
	let todos: Vec<Todo> = todos.into_iter().filter(|t| t.title.starts_with(fx_title_prefix)).collect();

	// -- Check
	assert_eq!(todos.len(), 3, "number of todos");

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
	let fx_title_prefix = "sb_select_ok_count";
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
