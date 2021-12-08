mod utils;

use sqlb::{Field, Raw, SqlBuilder};
use sqlx::types::time::OffsetDateTime;
use std::error::Error;
use utils::{init_db, util_fetch_all_todos, util_insert_todo};

use crate::utils::util_fetch_todo;

#[test]
#[should_panic]
fn sb_update_all_sql_panic() {
	let sb = sqlb::update().table("todo");
	sb.sql();
	// should panic
}

#[test]
fn sb_update_all_sql_ok() {
	let sb = sqlb::update_all().table("todo");
	sb.sql();
	// should pass
}

#[tokio::test]
async fn sb_update_all_exec() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let _todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let _todo_id_2 = util_insert_todo(test_title_2, &db_pool).await?;

	// DO update
	let test_title_for_all = "test - new title for all";
	let fields = vec![("title", test_title_for_all).into()];
	let sb = sqlb::update_all().table("todo").data(fields);
	// let row_affected = sqlx_exec::exec(&db_pool, &sb).await?;
	let row_affected = sb.exec(&db_pool).await?;
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

	// FIXTURES
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let todo_id_2 = util_insert_todo(test_title_2, &db_pool).await?;

	// DO update
	let test_title_for_all = "test - new title";
	let fields = vec![("title", test_title_for_all).into()];
	let sb = sqlb::update().table("todo").data(fields).and_where_eq("id", todo_id_1);

	let row_affected = sb.exec(&db_pool).await?;
	assert_eq!(1, row_affected, "row_affected");

	// CHECK todo_1
	let todo = util_fetch_todo(&db_pool, todo_id_1).await?;
	assert_eq!(test_title_for_all, todo.title, "todo_1.tile");

	// CHECK todo_2
	let todo = util_fetch_todo(&db_pool, todo_id_2).await?;
	assert_eq!(test_title_2, todo.title, "todo_1.tile");

	Ok(())
}

#[tokio::test]
async fn sb_update_exec_with_wheres() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let todo_id_2 = util_insert_todo(test_title_2, &db_pool).await?;

	// DO update
	let test_title_for_all = "test - new title";
	let fields = vec![("title", test_title_for_all).into()];
	let sb = sqlb::update()
		.table("todo")
		.data(fields)
		.and_where("id", "=", todo_id_1)
		.and_where("title", "=", test_title_1);
	let row_affected = sb.exec(&db_pool).await?;
	assert_eq!(1, row_affected, "row_affected");

	// CHECK todo_1
	let todo = util_fetch_todo(&db_pool, todo_id_1).await?;
	assert_eq!(test_title_for_all, todo.title, "todo_1.tile");

	// CHECK todo_2
	let todo = util_fetch_todo(&db_pool, todo_id_2).await?;
	assert_eq!(test_title_2, todo.title, "todo_1.tile");

	Ok(())
}

#[tokio::test]
async fn sb_update_returning() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title_1 = "test - title 01";
	let test_title_2 = "test - title 02";
	let todo_id_1 = util_insert_todo(test_title_1, &db_pool).await?;
	let todo_id_2 = util_insert_todo(test_title_2, &db_pool).await?;

	// DO update
	let test_title_new = "test - new title";
	let fields = vec![("title", test_title_new).into()];
	let sb = sqlb::update().table("todo").data(fields).and_where("id", "=", todo_id_1);
	let sb = sb.returning(&["id", "title"]);
	let (returned_todo_1_id, returned_todo_1_title) = sb.fetch_one::<(i64, String), _>(&db_pool).await?;

	// CHECK return values
	assert_eq!(todo_id_1, returned_todo_1_id);
	assert_eq!(test_title_new, returned_todo_1_title);

	// CHECK todo_2
	let todo = util_fetch_todo(&db_pool, todo_id_2).await?;
	assert_eq!(test_title_2, todo.title, "todo_1.tile");

	Ok(())
}

#[tokio::test]
async fn sb_update_raw() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURE
	let todo_id_1 = util_insert_todo("test_title_1", &db_pool).await?;
	let test_title_new = "test - new title";

	// ACTION
	let fields: Vec<Field> = vec![("title", test_title_new).into(), ("ctime", Raw("now()")).into()];
	let sb = sqlb::update().table("todo").data(fields).and_where_eq("id", todo_id_1);
	let sb = sb.returning(&["id", "title", "ctime"]);
	let (id, title, _ctime) = sb.fetch_one::<(i64, String, OffsetDateTime), _>(&db_pool).await?;

	// CHECK
	assert_eq!(test_title_new, title);
	assert_eq!(todo_id_1, id);

	Ok(())
}
