use crate::core::{add_to_where, into_returnings, sql_returnings, sql_where_items};
use crate::core::{WhereItem, Whereable};
use crate::utils::{x_column_name, x_table_name};
use crate::{sqlx_exec, Field, SqlBuilder, SqlxBindable};
use async_trait::async_trait;
use sqlx::{Executor, FromRow, Postgres};

pub fn update<'a>() -> UpdateSqlBuilder<'a> {
	UpdateSqlBuilder {
		guard_all: true,
		table: None,
		data: Vec::new(),
		returnings: None,
		and_wheres: Vec::new(),
	}
}

pub fn update_all<'a>() -> UpdateSqlBuilder<'a> {
	UpdateSqlBuilder {
		guard_all: false,
		table: None,
		data: Vec::new(),
		returnings: None,
		and_wheres: Vec::new(),
	}
}

pub struct UpdateSqlBuilder<'a> {
	guard_all: bool,
	table: Option<String>,
	data: Vec<Field<'a>>,
	returnings: Option<Vec<String>>,
	and_wheres: Vec<WhereItem<'a>>,
}

impl<'a> UpdateSqlBuilder<'a> {
	pub fn table(mut self, table: &str) -> Self {
		self.table = Some(table.to_string());
		self
	}

	pub fn data(mut self, fields: Vec<Field<'a>>) -> Self {
		self.data = fields;
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

	pub async fn exec<'q, E>(&'a self, db_pool: E) -> Result<u64, sqlx::Error>
	where
		E: Executor<'q, Database = Postgres>,
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

impl<'a> Whereable<'a> for UpdateSqlBuilder<'a> {
	fn and_where_eq<T: 'a + SqlxBindable + Send + Sync>(self, name: &str, val: T) -> Self {
		UpdateSqlBuilder::and_where_eq(self, name, val)
	}

	fn and_where<T: 'a + SqlxBindable + Send + Sync>(self, name: &str, op: &'static str, val: T) -> Self {
		UpdateSqlBuilder::and_where(self, name, op, val)
	}
}

#[async_trait]
impl<'a> SqlBuilder<'a> for UpdateSqlBuilder<'a> {
	fn sql(&self) -> String {
		// SQL: UPDATE table_name SET column1 = $1, ... WHERE w1 = $2, w2 = $3 returning r1, r2;

		// SQL: UPDATE table_name SET
		let mut sql = String::from("UPDATE ");

		if let Some(table) = &self.table {
			sql.push_str(&x_table_name(table));
		}

		sql.push_str("SET ");

		// Index for the $_idx_ in the prepared statement
		let mut binding_idx = 1;

		// TODO: Handle the case of empty data. Should we change this signature to return a Result ?
		//       For now, just ignore this case, will fail at sql exec time
		// SQL: column1 = $1, ...
		let fields = &self.data;
		let sql_set = fields
			.iter()
			.enumerate()
			.map(|(_, f)| {
				let mut part = format!("{} = ", x_column_name(&f.name));
				match f.value.raw() {
					None => {
						part.push_str(&format!("${}", binding_idx));
						binding_idx += 1;
					}
					Some(raw) => part.push_str(raw),
				}
				part
			})
			.collect::<Vec<String>>()
			.join(", ");
		sql.push_str(&format!("{} ", sql_set));

		// SQL: WHERE w1 < $1, ...
		if !self.and_wheres.is_empty() {
			let sql_where = sql_where_items(&self.and_wheres, binding_idx);
			sql.push_str(&format!("WHERE {} ", &sql_where));
		} else if self.guard_all {
			// For now panic, will return error later
			panic!("FATAL - Trying to call a update without any where clause. If needed, use sqlb::update_all(table_name). ")
		}

		// SQL: RETURNING "r1", "r2", ...
		if let Some(returnings) = &self.returnings {
			sql.push_str(&format!("RETURNING {} ", sql_returnings(returnings)));
		}

		sql
	}

	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send> {
		let iter = self.data.iter().map(|field| &field.value);
		// FIXME needs to uncomment
		let iter = iter.chain(self.and_wheres.iter().map(|wi| &wi.val));
		Box::new(iter)
	}

	async fn exec<'q, E>(&'a self, db_pool: E) -> Result<u64, sqlx::Error>
	where
		E: Executor<'q, Database = Postgres>,
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
