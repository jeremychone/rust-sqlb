mod utils;

use sqlb::{HasFields, SqlBuilder};
use std::error::Error;
use utils::{init_db, util_fetch_all_todos, util_insert_todo};

use crate::utils::Todo;
use serial_test::serial;

#[serial]
#[test]
#[should_panic]
fn sb_delete_err_all() {
	let sb = sqlb::delete().table("todo");
	sb.sql();
	// should panic
}

#[serial]
#[test]
fn sb_delete_ok_all() {
	let sb = sqlb::delete_all().table("todo");
	sb.sql();
	// should pass
}

#[serial]
#[tokio::test]
async fn sb_delete_ok_exec() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// -- Fixtures
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(&db_pool, test_title_1).await?;
	let _ = util_insert_todo(&db_pool, test_title_2).await?;

	// -- Exec
	// Note: test schema and fully qualified column name.
	let sb = sqlb::delete().table("public.todo").and_where("todo.id", "=", todo_id_1);
	let row_affected = sb.exec(&db_pool).await?;
	assert_eq!(1, row_affected, "row_affected");

	// -- Check - if only one todo_1 was deleted
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title_2, todos[0].title);

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_delete_ok_return_one() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// -- Fixtures
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(&db_pool, test_title_1).await?;
	let _ = util_insert_todo(&db_pool, test_title_2).await?;

	// -- Exec
	let sb = sqlb::delete().table("todo").and_where("id", "=", todo_id_1);
	let sb = sb.returning(&["id", "title"]);
	let (deleted_todo_1_id, deleted_todo_1_title) = sb.fetch_one::<_, (i64, String)>(&db_pool).await?;

	// -- Check - deleted returns
	assert_eq!(test_title_1, deleted_todo_1_title);
	assert_eq!(todo_id_1, deleted_todo_1_id);

	// -- Check - check with fetch all
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(1, todos.len());
	assert_eq!(test_title_2, todos[0].title);

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_delete_ok_return_many() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// -- Fixtures
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(&db_pool, test_title_1).await?;
	let todo_id_2 = util_insert_todo(&db_pool, test_title_2).await?;

	// -- Exec
	let sb = sqlb::delete().table("todo").and_where("id", ">", 0);
	let sb = sb.returning(Todo::field_names());

	let deleted: Vec<Todo> = sb.fetch_all(&db_pool).await?;

	// -- Check - deleted returns
	assert_eq!(2, deleted.len());
	assert_eq!(todo_id_1, deleted[0].id);
	assert_eq!(test_title_1, deleted[0].title);
	assert_eq!(todo_id_2, deleted[1].id);
	assert_eq!(test_title_2, deleted[1].title);

	// -- Check - empty table
	let todos = util_fetch_all_todos(&db_pool).await?;
	assert_eq!(0, todos.len());

	Ok(())
}
