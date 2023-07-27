use crate::core::{add_to_where, into_returnings, sql_returnings, sql_where_items};
use crate::core::{WhereItem, Whereable};
use crate::utils::x_table_name;
use crate::{sqlx_exec, SqlBuilder, SqlxBindable};
use async_trait::async_trait;
use sqlx::{Executor, FromRow, Postgres};

pub fn delete<'a>() -> DeleteSqlBuilder<'a> {
	DeleteSqlBuilder {
		guard_all: true,
		table: None,
		returnings: None,
		and_wheres: Vec::new(),
	}
}

pub fn delete_all<'a>() -> DeleteSqlBuilder<'a> {
	DeleteSqlBuilder {
		guard_all: false,
		table: None,
		returnings: None,
		and_wheres: Vec::new(),
	}
}

pub struct DeleteSqlBuilder<'a> {
	guard_all: bool,
	table: Option<String>,
	returnings: Option<Vec<String>>,
	and_wheres: Vec<WhereItem<'a>>,
}

impl<'a> DeleteSqlBuilder<'a> {
	pub fn table(mut self, table: &str) -> Self {
		self.table = Some(table.to_string());
		self
	}
	pub fn and_where<T: 'a + SqlxBindable + Send + Sync>(mut self, name: &str, op: &'static str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, op, val);
		self
	}

	pub fn and_where_eq<T: 'a + SqlxBindable + Send + Sync>(mut self, name: &str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, "=", val);
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returnings = into_returnings(self.returnings, names);
		self
	}

	pub async fn exec<'q, DB>(&'a self, db_pool: DB) -> Result<u64, sqlx::Error>
	where
		DB: Executor<'q, Database = Postgres>,
	{
		sqlx_exec::exec(db_pool, self).await
	}

	pub async fn fetch_one<'e, DB, D>(&'a self, db_pool: DB) -> Result<D, sqlx::Error>
	where
		DB: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		sqlx_exec::fetch_as_one::<DB, D, _>(db_pool, self).await
	}

	pub async fn fetch_optional<'e, DB, D>(&'a self, db_pool: DB) -> Result<Option<D>, sqlx::Error>
	where
		DB: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		sqlx_exec::fetch_as_optional::<DB, D, _>(db_pool, self).await
	}

	pub async fn fetch_all<'e, DB, D>(&'a self, db_pool: DB) -> Result<Vec<D>, sqlx::Error>
	where
		DB: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		sqlx_exec::fetch_as_all::<DB, D, _>(db_pool, self).await
	}
}

impl<'a> Whereable<'a> for DeleteSqlBuilder<'a> {
	fn and_where_eq<T: 'a + SqlxBindable + Send + Sync>(self, name: &str, val: T) -> Self {
		DeleteSqlBuilder::and_where_eq(self, name, val)
	}

	fn and_where<T: 'a + SqlxBindable + Send + Sync>(self, name: &str, op: &'static str, val: T) -> Self {
		DeleteSqlBuilder::and_where(self, name, op, val)
	}
}

#[async_trait]
impl<'a> SqlBuilder<'a> for DeleteSqlBuilder<'a> {
	fn sql(&self) -> String {
		// SQL: DELETE FROM table_name WHERE w1 = $1, ... RETURNING r1, r2, ..;

		// SQL: DELETE FROM table_name
		let mut sql = String::from("DELETE FROM ");

		if let Some(table) = &self.table {
			sql.push_str(&x_table_name(table));
		}

		// SQL: WHERE w1 < $1, ...
		if !self.and_wheres.is_empty() {
			let sql_where = sql_where_items(&self.and_wheres, 1);
			sql.push_str(&format!("WHERE {} ", &sql_where));
		} else if self.guard_all {
			// For now panic, will return error later
			panic!("FATAL - Trying to call a delete without any where clause. If needed, use sqlb::delete_all(table_name). ")
		}

		// SQL: RETURNING "r1", "r2", ...
		if let Some(returnings) = &self.returnings {
			sql.push_str(&format!("RETURNING {} ", sql_returnings(returnings)));
		}

		sql
	}

	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send> {
		let iter = self.and_wheres.iter().map(|wi| &wi.val);
		Box::new(iter)
	}

	async fn exec<'q, DB>(&'a self, db_pool: DB) -> Result<u64, sqlx::Error>
	where
		DB: Executor<'q, Database = Postgres>,
	{
		Self::exec(self, db_pool).await
	}

	async fn fetch_one<'e, DB, D>(&'a self, db_pool: DB) -> Result<D, sqlx::Error>
	where
		DB: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		Self::fetch_one::<DB, D>(self, db_pool).await
	}

	async fn fetch_optional<'e, DB, D>(&'a self, db_pool: DB) -> Result<Option<D>, sqlx::Error>
	where
		DB: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		Self::fetch_optional::<DB, D>(self, db_pool).await
	}

	async fn fetch_all<'e, DB, D>(&'a self, db_pool: DB) -> Result<Vec<D>, sqlx::Error>
	where
		DB: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	{
		Self::fetch_all::<DB, D>(self, db_pool).await
	}
}
