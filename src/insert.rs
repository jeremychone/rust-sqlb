use crate::core::{into_returnings, sql_comma_names, sql_comma_params, sql_returnings};
use crate::{Field, SqlBuilder, SqlxBindable};

pub fn insert(table: &str) -> SqlInsertBuilder {
	SqlInsertBuilder {
		table: table.to_string(),
		data: Vec::new(),
		returnings: None,
	}
}

// #[derive(Clone)]
pub struct SqlInsertBuilder<'a> {
	table: String,
	data: Vec<Field<'a>>,
	returnings: Option<Vec<String>>,
}

impl<'a> SqlInsertBuilder<'a> {
	pub fn data(mut self, fields: Vec<Field<'a>>) -> Self {
		self.data = fields;
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returnings = into_returnings(self.returnings, names);
		self
	}
}

impl<'a> SqlBuilder<'a> for SqlInsertBuilder<'a> {
	fn sql(&self) -> String {
		// SQL: INSERT INTO table_name (name1, ...) VALUES ($1, ...) RETURNING r1, ...;

		// SQL: INSERT INTO table_name
		let mut sql = String::from(format!("INSERT INTO \"{}\" ", self.table));

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
}
