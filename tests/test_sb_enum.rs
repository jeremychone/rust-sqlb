mod utils;

use serial_test::serial;
use sqlb::{Field, SqlxBindable};
use sqlx::{postgres::PgArguments, query::Query, Postgres};
use std::error::Error;
use utils::init_db;

// region:    Custom Type (enum)
#[derive(Eq, PartialEq, Hash, sqlx::Type, Debug, Clone)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
	New,
	Open,
	Done,
}

// NOTE: manual implementation, see test_rules for the macros alternative.
impl SqlxBindable for TodoStatus {
	fn bind_query<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
		let query = query.bind(self.clone());
		query
	}
}

// endregion: Custom Type (enum)

// This is to test that the type above was undestood by Sqlx
#[serial]
#[tokio::test]
async fn sb_enum_direct_sqlx_ok() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	let query = sqlx::query::<sqlx::Postgres>("INSERT INTO todo (title, status) VALUES ($1, $2)");
	let query = query.bind("test sb_enum_direct_sqlx_ok 01");
	let query = query.bind(TodoStatus::Done);
	let _r = query.execute(&db_pool).await?.rows_affected();

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_enum_direct_sqlb_field_ok() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// fixtures
	let title_1 = "test - sb_enum_direct_sqlb_field_ok";
	let status_1 = TodoStatus::Open;

	// DO the insert
	let data: Vec<Field> = vec![("title", title_1).into(), ("status", TodoStatus::Open).into()];
	let sb = sqlb::insert().table("todo").data(data).returning(&["id", "title", "status"]);
	let (id_1, title, status) = sb.fetch_one::<_, (i64, String, TodoStatus)>(&db_pool).await?;

	// CHECK the insert
	assert_eq!(title_1, title);
	assert_eq!(status_1, status);

	// DO the select
	let sb = sqlb::select()
		.table("todo")
		.columns(&["id", "title", "status"])
		.and_where_eq("id", id_1);
	let (id, title, status) = sb.fetch_one::<_, (i64, String, TodoStatus)>(&db_pool).await?;

	// CHECK the insert
	assert_eq!(id_1, id);
	assert_eq!(title_1, title);
	assert_eq!(status_1, status);

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_enum_option_sqlx_ok() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	let status = Some(TodoStatus::Done);
	let query = sqlx::query::<sqlx::Postgres>("INSERT INTO todo (title, status) VALUES ($1, $2)");
	let query = query.bind("test sb_enum_option_sqlx_ok 01");
	let query = query.bind(status);
	let _r = query.execute(&db_pool).await?.rows_affected();

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_enum_option_sqlb_field_ok() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// fixtures
	let title_1 = "test - sb_enum_option_sqlb_field_ok";
	let status_1 = TodoStatus::Open;

	// DO the insert
	let status = Some(TodoStatus::Open);
	let data: Vec<Field> = vec![("title", title_1).into(), ("status", status).into()];
	let sb = sqlb::insert().table("todo").data(data).returning(&["id", "title", "status"]);
	let (id_1, title, status) = sb.fetch_one::<_, (i64, String, TodoStatus)>(&db_pool).await?;

	// CHECK the insert
	assert_eq!(title_1, title);
	assert_eq!(status_1, status);

	// DO the select
	let sb = sqlb::select()
		.table("todo")
		.columns(&["id", "title", "status"])
		.and_where_eq("id", id_1);
	let (id, title, status) = sb.fetch_one::<_, (i64, String, TodoStatus)>(&db_pool).await?;

	// CHECK the insert
	assert_eq!(id_1, id);
	assert_eq!(title_1, title);
	assert_eq!(status_1, status);

	Ok(())
}
