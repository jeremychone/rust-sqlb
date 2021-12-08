use crate::core::{into_returnings, sql_comma_names, sql_comma_params, sql_returnings};
use crate::{sqlx_exec, Field, SqlBuilder, SqlxBindable};
use async_trait::async_trait;
use sqlx::{Executor, FromRow, Postgres};

pub fn insert<'a>() -> InsertSqlBuilder<'a> {
	InsertSqlBuilder {
		table: None,
		data: Vec::new(),
		returnings: None,
	}
}

// #[derive(Clone)]
pub struct InsertSqlBuilder<'a> {
	table: Option<String>,
	data: Vec<Field<'a>>,
	returnings: Option<Vec<String>>,
}

impl<'a> InsertSqlBuilder<'a> {
	pub fn table(mut self, table: &str) -> Self {
		self.table = Some(table.to_string());
		self
	}

	pub fn data(mut self, fields: Vec<Field<'a>>) -> Self {
		self.data = fields;
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returnings = into_returnings(self.returnings, names);
		self
	}

	pub async fn exec<'q, E>(&'a self, db_pool: E) -> Result<u64, sqlx::Error>
	where
		E: Executor<'q, Database = Postgres>,
	{
		sqlx_exec::exec(db_pool, self).await
	}

	pub async fn fetch_one<'e, D, E>(&'a self, db_pool: E) -> Result<D, sqlx::Error>
	where
		E: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		sqlx_exec::fetch_as_one::<D, E, _>(db_pool, self).await
	}

	pub async fn fetch_all<'e, D, E>(&'a self, db_pool: E) -> Result<Vec<D>, sqlx::Error>
	where
		E: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		sqlx_exec::fetch_as_all::<D, E, _>(db_pool, self).await
	}
}

#[async_trait]
impl<'a> SqlBuilder<'a> for InsertSqlBuilder<'a> {
	fn sql(&self) -> String {
		// SQL: INSERT INTO table_name (name1, ...) VALUES ($1, ...) RETURNING r1, ...;

		// SQL: INSERT INTO table_name
		let mut sql = String::from("INSERT INTO ");

		if let Some(table) = &self.table {
			sql.push_str(&format!("\"{}\" ", table));
		}

		// Note: empty data is a valid usecase, if the row has a all required field with default or auto gen.
		let fields = &self.data;
		// SQL: (name1, name2, ...)
		sql.push_str(&format!("({}) ", sql_comma_names(fields)));

		// SQL: VALUES ($1, $2, ...)
		sql.push_str(&format!("VALUES ({}) ", sql_comma_params(fields).1));

		// SQL: RETURNING "r1", "r2", ...
		if let Some(returnings) = &self.returnings {
			sql.push_str(&format!("RETURNING {} ", sql_returnings(returnings)));
		}

		sql
	}

	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send> {
		let iter = self.data.iter().map(|field| &field.1);
		Box::new(iter)
	}

	async fn exec<'q, E>(&'a self, db_pool: E) -> Result<u64, sqlx::Error>
	where
		E: Executor<'q, Database = Postgres>,
	{
		Self::exec(self, db_pool).await
	}

	async fn fetch_one<'e, D, E>(&'a self, db_pool: E) -> Result<D, sqlx::Error>
	where
		E: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		Self::fetch_one::<D, E>(self, db_pool).await
	}

	async fn fetch_all<'e, D, E>(&'a self, db_pool: E) -> Result<Vec<D>, sqlx::Error>
	where
		E: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		Self::fetch_all::<D, E>(self, db_pool).await
	}
}
