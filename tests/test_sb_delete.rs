mod utils;

use sqlb::SqlBuilder;
use std::error::Error;
use utils::{init_db, util_fetch_all_todos, util_insert_todo};

use crate::utils::Todo;

#[test]
#[should_panic]
fn sb_delete_all_panic() {
	let sb = sqlb::delete().table("todo");
	sb.sql();
	// should panic
}

#[test]
fn sb_delete_all_ok() {
	let sb = sqlb::delete_all().table("todo");
	sb.sql();
	// should pass
}

#[tokio::test]
async fn sb_delete_exec() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let _ = util_insert_todo(test_title_2, &db_pool).await?;

	// DO the delete
	let sb = sqlb::delete().table("todo").and_where("id", "=", todo_id_1);
	let row_affected = sb.exec(&db_pool).await?;
	assert_eq!(1, row_affected, "row_affected");

	// CHECK if only one todo_1 was deleted
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title_2, todos[0].title);

	Ok(())
}

#[tokio::test]
async fn sb_delete_return_one() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let _ = util_insert_todo(test_title_2, &db_pool).await?;

	// DO delete
	let sb = sqlb::delete().table("todo").and_where("id", "=", todo_id_1);
	let sb = sb.returning(&["id", "title"]);
	let (deleted_todo_1_id, deleted_todo_1_title) = sb.fetch_one::<(i64, String), _>(&db_pool).await?;

	// CHECK deleted returns
	assert_eq!(test_title_1, deleted_todo_1_title);
	assert_eq!(todo_id_1, deleted_todo_1_id);

	// CHECK check with fetch all
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title_2, todos[0].title);

	Ok(())
}

#[tokio::test]
async fn sb_delete_return_many() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let todo_id_2 = util_insert_todo(test_title_2, &db_pool).await?;

	// DO delete
	let sb = sqlb::delete().table("todo").and_where("id", ">", 0);
	let sb = sb.returning(&["id", "title"]);

	let deleted: Vec<Todo> = sb.fetch_all(&db_pool).await?;

	// CHECK deleted returns
	assert_eq!(2, deleted.len());
	assert_eq!(todo_id_1, deleted[0].id);
	assert_eq!(test_title_1, deleted[0].title);
	assert_eq!(todo_id_2, deleted[1].id);
	assert_eq!(test_title_2, deleted[1].title);

	// CHECK empty table
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(0, todos.len());

	Ok(())
}
