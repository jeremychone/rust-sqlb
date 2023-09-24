use crate::utils::Todo;
use anyhow::Result;
use sea_query::{Expr, Iden, IntoIden, PostgresQueryBuilder, Query};
use sqlb::{Field, HasFields, SIden};

mod utils;

#[test]
fn test_sea_query_fields_idens() -> Result<()> {
	let columns = Todo::field_idens();

	let (_sql, _values) = Query::select().from(SIden("todo")).columns(columns).build(PostgresQueryBuilder);

	Ok(())
}

#[test]
fn test_sea_query_fields_unzip_columns_values() -> Result<()> {
	let todo = Todo::default();
	let fields = todo.not_none_fields();

	let (columns, values) = fields.unzip();

	let (_sql, _values) = Query::insert()
		.into_table(SIden("todo"))
		.columns(columns)
		.values(values)?
		.returning(Query::returning().columns([SIden("id")]))
		.build(PostgresQueryBuilder);

	Ok(())
}

#[test]
fn test_fields_into_tuple_iter() -> Result<()> {
	let todo = Todo::default();
	let fields = todo.not_none_fields().zip();

	let (_sql, _values) = Query::update()
		.table(SIden("todo"))
		.values(fields)
		.and_where(Expr::col(SIden("id")).eq(123))
		.build(PostgresQueryBuilder);

	Ok(())
}

#[test]
fn test_fields_add_custom_name_value() -> Result<()> {
	let todo = Todo::default();
	let mut fields = todo.not_none_fields();
	fields.push(Field::new(TimestampSpec::Cid.into_iden(), 123.into()));

	Ok(())
}

#[test]
fn test_fields_add_custom_field_iden() -> Result<()> {
	let mut columns = Todo::field_idens();
	columns.push(TimestampSpec::Cid.into_iden());

	let (_sql, _values) = Query::select().from(SIden("todo")).columns(columns).build(PostgresQueryBuilder);

	Ok(())
}

#[derive(Iden)]
enum TimestampSpec {
	Cid,
}
