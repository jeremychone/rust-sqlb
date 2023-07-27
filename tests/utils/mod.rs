#![allow(unused)]

use anyhow::Result;
use sqlb::{Field, HasFields};
use sqlb_macros::Fields;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// region:    Test Types
#[derive(Debug, sqlx::FromRow, Fields)]
pub struct Todo {
	pub id: i64,
	pub title: String,
	#[sqlx(rename = "description")]
	#[field(name = "description")]
	pub desc: Option<String>,
}

#[derive(Debug, sqlb::Fields)]
pub struct TodoPatch {
	pub title: Option<String>,
	#[field(name = "description")]
	pub desc: Option<String>,
}

// -- Manual implementation
// impl HasFields for TodoPatch {
// 	fn not_none_fields<'a>(self) -> Vec<Field<'a>> {
// 		let mut fields = Vec::new();
// 		if let Some(title) = self.title {
// 			fields.push(("title", title).into());
// 		}
// 		fields
// 	}

// 	#[allow(clippy::vec_init_then_push)]
// 	fn all_fields<'a>(self) -> Vec<Field<'a>> {
// 		let mut fields: Vec<Field> = Vec::new();
// 		fields.push(("title", self.title).into());
// 		fields
// 	}

// 	fn field_names() -> &'static [&'static str] {
// 		&["title"]
// 	}
// }

// endregion: Test Types

// region:    Test Seed Utils

pub async fn util_insert_many_todos(db_pool: &Pool<Postgres>, title_prefix: &str, count: i32) -> Result<Vec<i64>> {
	let mut ids = Vec::new();

	for idx in 0..count {
		let title = format!("{title_prefix}-{:0>2}", idx);
		let id = util_insert_todo(db_pool, &title).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn util_insert_todos(db_pool: &Pool<Postgres>, titles: &[&str]) -> Result<Vec<i64>> {
	let mut ids = Vec::new();

	for title in titles {
		let id = util_insert_todo(db_pool, title).await?;
		ids.push(id);
	}

	Ok(ids)
}

pub async fn util_insert_todo(db_pool: &Pool<Postgres>, title: &str) -> Result<i64> {
	let patch_data = TodoPatch {
		title: Some(title.to_string()),
		desc: None,
	};

	let sb = sqlb::insert().table("todo").data(patch_data.not_none_fields());
	let sb = sb.returning(&["id"]);
	let (id,) = sb.fetch_one::<_, (i64,)>(db_pool).await?;

	Ok(id)
}

pub async fn util_fetch_all_todos(db_pool: &Pool<Postgres>) -> Result<Vec<Todo>> {
	let sb = sqlb::select()
		.table("todo")
		.columns(&["id", "title", "description"])
		.order_by("!id");
	let todos = sb.fetch_all::<_, Todo>(db_pool).await?;
	Ok(todos)
}

pub async fn util_fetch_todo(db_pool: &Pool<Postgres>, id: i64) -> Result<Todo> {
	let sb = sqlb::select()
		.table("todo")
		.columns(&["id", "title", "description"])
		.and_where("id", "=", id);
	let todos = sb.fetch_one::<_, Todo>(db_pool).await?;
	Ok(todos)
}
// endregion: Test Seed Utils

// region:    Test Utils
pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect("postgres://postgres:welcome@localhost/postgres")
		.await?;

	sqlx::query("DROP TABLE IF EXISTS todo").execute(&pool).await?;

	// create todo status
	if let Err(ex) = sqlx::query("DROP TYPE todo_status_enum").execute(&pool).await {
		println!("Warning - {}", ex);
	}
	if let Err(ex) = sqlx::query(
		r#"
CREATE TYPE todo_status_enum AS ENUM (
  'new',
  'open',
	'done'
);	
	"#,
	)
	.execute(&pool)
	.await
	{
		println!("ERROR CREATE TYPE todo_status_enum - {}", ex);
	}

	// Create todo table

	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS todo (
  id bigserial,
  title text,
	description text,
	ctime timestamp with time zone,
	"desc" text,
	status todo_status_enum
);"#,
	)
	.execute(&pool)
	.await?;

	// Create project table
	sqlx::query("DROP TABLE IF EXISTS projects").execute(&pool).await?;
	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS project (
  id bigserial,
  name text
);"#,
	)
	.execute(&pool)
	.await?;

	Ok(pool)
}

// endregion: Test Utils
