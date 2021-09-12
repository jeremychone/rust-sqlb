#![allow(unused)] // silence unused warnings while exploring (to comment out)

mod utils;

use sqlb::{insert, select, sqlx_exec, Field, SqlBuilder, SqlxBindable};
use sqlx::{
	postgres::{PgArgumentBuffer, PgArguments},
	query::Query,
	query_with, Arguments, Encode, Execute, IntoArguments, Postgres, Type,
};
use std::{any::Any, error::Error};
use utils::{init_db, util_fetch_all_todos, TodoPatch};

// region:    Custom Type (enum)
#[derive(Eq, PartialEq, Hash, sqlx::Type, Debug, Copy, Clone)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
	New,
	Open,
	Done,
}

impl<'a> SqlxBindable for TodoStatus {
	fn bind_query<'q>(&self, mut query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
		let query = query.bind(*self);
		query
	}
}

// endregion: Custom Type (enum)

// THis is to test that the type above was undestood by Sqlx
#[tokio::test]
async fn sb_enum_sqlx_raw_bind() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	let query = sqlx::query::<sqlx::Postgres>("INSERT INTO todo (title, status) VALUES ($1, $2)");
	let query = query.bind("test sb_enum_insert 01");
	let query = query.bind(TodoStatus::Done);
	let r = query.execute(&db_pool).await?.rows_affected();

	Ok(())
}

#[tokio::test]
async fn sb_enum_insert_() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// fixtures
	let title_1 = "test - sb_enum_insert_sqlb";
	let status_1 = TodoStatus::Open;

	// DO the insert
	let mut data: Vec<Field> = vec![("title", title_1).into(), ("status", TodoStatus::Open).into()];
	let sb = insert("todo").data(data).returning(&["id", "title", "status"]);
	let (id_1, title, status) = sqlx_exec::fetch_as_one::<(i64, String, TodoStatus), _, _>(&db_pool, &sb).await?;

	// CHECK the insert
	assert_eq!(title_1, title);
	assert_eq!(status_1, status);

	// DO the select
	let sb = select("todo").columns(&["id", "title", "status"]).and_where_eq("id", id_1);
	let (id, title, status) = sqlx_exec::fetch_as_one::<(i64, String, TodoStatus), _, _>(&db_pool, &sb).await?;

	// CHECK the insert
	assert_eq!(id_1, id);
	assert_eq!(title_1, title);
	assert_eq!(status_1, status);

	Ok(())
}
