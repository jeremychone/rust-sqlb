//! Currently, `SqlxBindable` represents a value that can be bound.
//! This requires cloning the value. The performance impact should be minimal, and for bulk updates, direct usage of `sqlx` can be preferred.
//! Eventually, this might change to follow the `'args` pattern of `sqlx` `Builder`,
//! but at this point, priority is given to API ergonomics.
//!

use time::OffsetDateTime;
use uuid::Uuid;

pub trait SqlxBindable: std::fmt::Debug {
	fn bind_query<'q>(
		&'q self,
		query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
	) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>;

	fn raw(&self) -> Option<&str> {
		None
	}
}

#[macro_export]
macro_rules! bindable {
    ($($t:ty),*) => {
        $(impl $crate::SqlxBindable for $t {
            fn bind_query<'q>(&self, query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
                let query = query.bind(self.clone());
                query
            }
        }

        impl $crate::SqlxBindable for &$t {
            fn bind_query<'q>(&self, query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
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
		$(
		impl $crate::SqlxBindable for $t {
			fn bind_query<'q>(&self, query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
				let query = query.bind(self.to_string());
				query
			}
		}

		impl $crate::SqlxBindable for &$t {
			fn bind_query<'q>(&self, query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
				let query = query.bind(self.to_string());
				query
			}
		}
		)*
	};
}

// Bind the string types
bindable_to_string!(String, str);

impl<T> SqlxBindable for Option<T>
where
	T: SqlxBindable + Clone + Send,
	T: for<'r> sqlx::Encode<'r, sqlx::Postgres>,
	T: sqlx::Type<sqlx::Postgres>,
{
	fn bind_query<'q>(
		&'q self,
		query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
	) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
		let query = query.bind(self.clone());
		query
	}
}

// Bind the boolean
bindable!(bool);
// Bind the numbers
// NOTE: Skipping u8, u16, u64 since not mapped by sqlx to postgres.
bindable!(i8, i16, i32, i64, f32, f64);

bindable!(Uuid, OffsetDateTime);

// region:    --- Raw Value

// region: 		--- chrono support
#[cfg(feature = "chrono-support")]
mod chrono_support {
	use chrono::{NaiveDateTime, NaiveDate, NaiveTime, DateTime, Utc};

	bindable!(NaiveDateTime, NaiveDate, NaiveTime, DateTime<Utc>);
}
// endregion: --- chrono support


#[derive(Debug)]
pub struct Raw(pub &'static str);

impl SqlxBindable for Raw {
	// just return the query given, since no binding should be taken place
	fn bind_query<'q>(
		&self,
		query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
	) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
		query
	}

	fn raw(&self) -> Option<&str> {
		Some(self.0)
	}
}
// endregion: --- Raw Value

#[cfg(test)]
mod tests {
	use crate::Field;

	#[test]
	fn field_from_str() {
		let field = Field::from(("name1", "v2"));
		assert_eq!("name1", field.name);

		let field: Field = ("name1", "v2").into();
		assert_eq!("name1", field.name);
	}

	#[test]
	fn field_from_string() {
		let field = Field::from(("name1", "v1"));
		assert_eq!("name1", field.name);

		let v2 = &"v2".to_string();
		let field: Field = ("name2", v2).into();
		assert_eq!("name2", field.name);
	}
}
