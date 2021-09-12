////////////////////////////////////
// sqlx-exec - module for the sqlx query executor
////

use sqlx::{
	postgres::PgArguments,
	query::{Query, QueryAs},
	Execute, Executor, FromRow, Postgres,
};

use crate::{val::Field, SqlBuilder};

/// Build a sqlx::query_as for the D (Data) generic type, binds the values, and does a .fetch_one and returns E
pub async fn fetch_as_one<'q, D, E, Q>(db_exec: E, sb: &'q Q) -> Result<D, sqlx::Error>
where
	E: Executor<'q, Database = Postgres>,
	D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	Q: SqlBuilder<'q>,
{
	let sql = sb.sql();
	let vals = sb.vals();

	// build temp query for binding
	let mut query = sqlx::query::<sqlx::Postgres>(&sql);
	for val in vals.into_iter() {
		query = val.bind_query(query);
	}

	// create the QueryAs
	let query = sqlx::query_as_with::<sqlx::Postgres, D, PgArguments>(&sql, query.take_arguments().unwrap());

	// exec and return
	let r = query.fetch_one(db_exec).await?;
	Ok(r)
}

/// Build a sqlx::query_as for the D (Data) generic type, binds the values, and does a .fetch_all and returns Vec<E>
pub async fn fetch_as_all<'q, D, E, Q>(db_exec: E, sb: &'q Q) -> Result<Vec<D>, sqlx::Error>
where
	D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	E: Executor<'q, Database = Postgres>,
	Q: SqlBuilder<'q>,
{
	let sql = sb.sql();
	let vals = sb.vals();

	// build temp query for binding
	let mut query = sqlx::query::<sqlx::Postgres>(&sql);
	for val in vals.into_iter() {
		query = val.bind_query(query);
	}

	// create the QueryAs
	let query = sqlx::query_as_with::<sqlx::Postgres, D, PgArguments>(&sql, query.take_arguments().unwrap());

	// exec and return
	let r = query.fetch_all(db_exec).await?;
	Ok(r)
}

pub async fn exec<'q, Q, E>(db_exec: E, sb: &'q Q) -> Result<u64, sqlx::Error>
where
	Q: SqlBuilder<'q>,
	E: Executor<'q, Database = Postgres>,
{
	let sql = sb.sql();
	let vals = sb.vals();
	let mut query = sqlx::query::<sqlx::Postgres>(&sql);
	for val in vals.into_iter() {
		query = val.bind_query(query);
	}

	let r = query.execute(db_exec).await?.rows_affected();

	Ok(r)
}
