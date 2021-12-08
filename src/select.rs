use crate::core::{add_to_where, sql_where_items, x_name, Whereable};
use crate::core::{OrderItem, WhereItem};
use crate::sqlx_exec;
use crate::{SqlBuilder, SqlxBindable};
use async_trait::async_trait;
use sqlx::{Executor, FromRow, Postgres};

pub fn select<'a>() -> SelectSqlBuilder<'a> {
	SelectSqlBuilder {
		table: None,
		columns: None,
		and_wheres: Vec::new(),
		order_bys: None,
	}
}

pub struct SelectSqlBuilder<'a> {
	table: Option<String>,
	columns: Option<Vec<String>>,
	// TODO: needs to support full condition (and/or)
	and_wheres: Vec<WhereItem<'a>>,
	order_bys: Option<Vec<OrderItem>>,
}

impl<'a> SelectSqlBuilder<'a> {
	pub fn and_where_eq<T: 'a + SqlxBindable + Send + Sync>(mut self: Self, name: &str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, "=", val);
		self
	}

	pub fn and_where<T: 'a + SqlxBindable + Send + Sync>(mut self, name: &str, op: &'static str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, op, val);
		self
	}

	pub fn table(mut self, table: &str) -> Self {
		self.table = Some(table.to_string());
		self
	}

	pub fn columns(mut self, names: &[&str]) -> Self {
		self.columns = Some(names.into_iter().map(|s| s.to_string()).collect());
		self
	}

	pub fn order_bys(mut self, odrs: &[&str]) -> Self {
		self.order_bys = Some(odrs.to_vec().into_iter().map(|o| o.into()).collect());
		self
	}

	pub fn order_by(mut self, odr: &str) -> Self {
		self.order_bys = Some(vec![odr.into()]);
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

impl<'a> Whereable<'a> for SelectSqlBuilder<'a> {
	fn and_where_eq<V: 'a + SqlxBindable + Send + Sync>(self: Self, name: &str, val: V) -> Self {
		SelectSqlBuilder::and_where_eq(self, name, val)
	}

	fn and_where<V: 'a + SqlxBindable + Send + Sync>(self: Self, name: &str, op: &'static str, val: V) -> Self {
		SelectSqlBuilder::and_where(self, name, op, val)
	}
}

#[async_trait]
impl<'a> SqlBuilder<'a> for SelectSqlBuilder<'a> {
	fn sql(&self) -> String {
		// SELECT name1, name2 FROM table_name WHERE w1 < r1, w2 = r2

		// SQL: SELECT
		let mut sql = String::from("SELECT ");

		// SQL: name1, name2,
		// For now, if no column, will do a "*"
		match &self.columns {
			Some(columns) => {
				let names = columns.iter().map(|c| x_name(c)).collect::<Vec<String>>().join(", ");
				sql.push_str(&format!("{} ", names));
			}
			None => sql.push_str(&format!("{} ", "*")),
		};

		// SQL: FROM table_name
		if let Some(table) = &self.table {
			sql.push_str(&format!("FROM {} ", x_name(table)));
		}

		// SQL: WHERE w1 < $1, ...
		if self.and_wheres.len() > 0 {
			let sql_where = sql_where_items(&self.and_wheres, 1);
			sql.push_str(&format!("WHERE {} ", &sql_where));
		}

		// SQL: ORDER BY
		if let Some(order_bys) = &self.order_bys {
			let sql_order_bys = order_bys.iter().map::<String, _>(|o| o.into()).collect::<Vec<String>>().join(", ");
			sql.push_str(&format!("ORDER BY {} ", sql_order_bys))
		}

		sql
	}

	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send> {
		let iter = self.and_wheres.iter().map(|wi| &wi.val);
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
