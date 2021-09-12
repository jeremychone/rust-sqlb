use std::{pin::Pin, rc::Rc, sync::Arc};

use sqlx::{
	postgres::PgArguments,
	query::{Query, QueryAs},
	Arguments, Encode, Postgres, Type,
};

pub trait SqlxBindable {
	fn bind_query<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments>;
}

// region:    Field
// #[derive(Clone)]
pub struct Field<'a>(pub String, pub Box<dyn SqlxBindable + 'a + Send + Sync>);

pub trait GetFields {
	fn fields(&self) -> Vec<Field>;
}

//// (&str, SqlxBindable) into Field(String, Box<dyn SqlxBindable>)
impl<'a, T: 'a + SqlxBindable + Send + Sync> From<(&str, T)> for Field<'a> {
	fn from((name, value): (&str, T)) -> Self {
		Field(name.to_owned(), Box::new(value))
	}
}
// endregion: field

impl<'a> SqlxBindable for String {
	fn bind_query<'q>(&self, mut query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
		let query = query.bind(self.to_owned());
		query
	}
}

impl<'a> SqlxBindable for &String {
	fn bind_query<'q>(&self, mut query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
		let query = query.bind(self.to_string());
		query
	}
}

impl<'a> SqlxBindable for &str {
	fn bind_query<'q>(&self, mut query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
		let query = query.bind(self.to_string());
		query
	}
}

impl<'a> SqlxBindable for i64 {
	fn bind_query<'q>(&self, mut query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
		let query = query.bind(*self);
		query
	}
}

impl<'a> SqlxBindable for &i64 {
	fn bind_query<'q>(&self, mut query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
		let query = query.bind(**self);
		query
	}
}

#[cfg(test)]
mod tests {
	use crate::val::Field;

	#[test]
	fn field_from_str() {
		let field = Field::from(("name1", "v2"));
		assert_eq!("name1", field.0);

		let field: Field = ("name1", "v2").into();
		assert_eq!("name1", field.0);
	}

	#[test]
	fn field_from_string() {
		let field = Field::from(("name1", "v1"));
		assert_eq!("name1", field.0);

		let v2 = &"v2".to_string();
		let field: Field = ("name2", v2).into();
		assert_eq!("name2", field.0);
	}
}
