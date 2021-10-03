use sqlx::{postgres::PgArguments, query::Query, Postgres};

pub trait SqlxBindable {
	fn bind_query<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments>;
}

// CONSIDER: Might want to consolidate those macros to take a first params as the tile of binding

#[macro_export]
macro_rules! bindable {
	($($t:ident),*) => {
		$(impl $crate::SqlxBindable for $t {
			fn bind_query<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
				let query = query.bind(self.clone());
				query
			}
		}
		impl $crate::SqlxBindable for &$t {
			fn bind_query<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
				let query = query.bind(<$t>::clone(self));
				query
			}
		}
		)*
	};
}

#[macro_export]
macro_rules! bindable_to_string {
	($($t:ident),*) => {
		$(impl $crate::SqlxBindable for $t {
			fn bind_query<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
				let query = query.bind(self.to_string());
				query
			}
		}
		impl $crate::SqlxBindable for &$t {
			fn bind_query<'q>(&self, query: Query<'q, Postgres, PgArguments>) -> Query<'q, sqlx::Postgres, PgArguments> {
				let query = query.bind(self.to_string());
				query
			}
		}
		)*
	};
}

// Bind the numbers
// NOTE: Skipping u8, u16, u64 since not mapped by sqlx to postgres
bindable!(i8, i16, i32, i64, u32, f32, f64);
// Bind the string types
bindable_to_string!(String, str);

#[cfg(test)]
mod tests {
	use crate::Field;

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
