////////////////////////////////////
// sqlx-exec - module for the sqlx query executor
////

use sqlx::{postgres::PgArguments, query::QueryAs, FromRow, Pool, Postgres};

use crate::{SqlBuilder, Val};

pub async fn fetch_as_one<'q, E, Q>(db_pool: &Pool<sqlx::Postgres>, sb: &Q) -> Result<E, sqlx::Error>
where
	E: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	Q: SqlBuilder,
{
	let sql = sb.sql();
	let vals = sb.vals();
	let query = sqlx::query_as::<sqlx::Postgres, E>(&sql);
	let query = sqlx_bind_vals(query, vals);
	let r = query.fetch_one(db_pool).await?;
	Ok(r)
}

pub async fn fetch_as_all<'q, E, Q>(db_pool: &Pool<sqlx::Postgres>, sb: &Q) -> Result<Vec<E>, sqlx::Error>
where
	E: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send,
	Q: SqlBuilder,
{
	let sql = sb.sql();
	let vals = sb.vals();
	let query = sqlx::query_as::<sqlx::Postgres, E>(&sql);
	let query = sqlx_bind_vals(query, vals);
	let r = query.fetch_all(db_pool).await?;
	Ok(r)
}

pub fn sqlx_bind_vals<'q, E>(mut query: QueryAs<'q, Postgres, E, PgArguments>, vals: Vec<Val>) -> QueryAs<'q, sqlx::Postgres, E, PgArguments> {
	for val in vals.into_iter() {
		query = sqlx_bind_val(query, val);
	}
	query
}

pub fn sqlx_bind_val<'q, E>(mut query: QueryAs<'q, Postgres, E, PgArguments>, val: Val) -> QueryAs<'q, sqlx::Postgres, E, PgArguments> {
	match val {
		Val::STRING(val) => query = query.bind(val),
		Val::I64(val) => query = query.bind(val),
	};
	query
}
