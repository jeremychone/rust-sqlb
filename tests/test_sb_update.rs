mod utils;

use sqlb::sqlx_exec;
use std::error::Error;
use utils::{init_db, util_fetch_all_todos, util_insert_todo};

use crate::utils::util_fetch_todo;

#[tokio::test]
async fn sb_update_exec_no_where() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// fixtures
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let _todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let _todo_id_2 = util_insert_todo(test_title_2, &db_pool).await?;

	// do update
	let test_title_for_all = "test - new title for all";
	let fields = vec![("title", test_title_for_all).into()];
	let sb = sqlb::update("todo").data(fields);
	let row_affected = sqlx_exec::exec(&db_pool, &sb).await?;
	assert_eq!(2, row_affected, "row_affected");

	// CHECK with select
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(2, todos.len());
	assert_eq!(test_title_for_all, todos[0].title, "todo.tile");
	assert_eq!(test_title_for_all, todos[1].title, "todo.tile");

	Ok(())
}

#[tokio::test]
async fn sb_update_exec_with_where() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// fixtures
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let todo_id_2 = util_insert_todo(test_title_2, &db_pool).await?;

	// do update
	let test_title_for_all = "test - new title";
	let fields = vec![("title", test_title_for_all).into()];
	let sb = sqlb::update("todo").data(fields).and_where(&[("id", "=", todo_id_1.into())]);
	let row_affected = sqlx_exec::exec(&db_pool, &sb).await?;
	assert_eq!(1, row_affected, "row_affected");

	// CHECK todo_1
	let todo = util_fetch_todo(&db_pool, todo_id_1).await?;
	assert_eq!(test_title_for_all, todo.title, "todo_1.tile");

	// CHECK todo_2
	let todo = util_fetch_todo(&db_pool, todo_id_2).await?;
	assert_eq!(test_title_2, todo.title, "todo_1.tile");

	Ok(())
}
