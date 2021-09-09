use sqlb::{Field, GetFields};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// region:    Test Types
#[derive(sqlx::FromRow)]
pub struct Todo {
	pub id: i64,
	pub title: String,
}

pub struct TodoPatch {
	pub title: Option<String>,
}

impl GetFields for TodoPatch {
	fn fields(&self) -> Vec<Field> {
		let mut fields = Vec::new();
		if let Some(title) = &self.title {
			fields.push(Field::from_string("title", title));
		}
		fields
	}
}
// endregion: Test Types

// region:    Test Utils
pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect("postgres://postgres:welcome@localhost/postgres")
		.await?;

	// Create todo table
	sqlx::query("DROP TABLE IF EXISTS todo").execute(&pool).await?;
	sqlx::query(
		r#"
CREATE TABLE IF NOT EXISTS todo (
  id bigserial,
  title text
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
