use crate::{sql_comma_names, sql_comma_params, Field, SqlBuilder, Val};

pub fn insert(table: &str) -> SqlInsertBuilder {
	SqlInsertBuilder {
		table: table.to_string(),
		data: None,
		returning: None,
	}
}

#[derive(Clone)]
pub struct SqlInsertBuilder {
	table: String,
	data: Option<Vec<Field>>,
	returning: Option<Vec<String>>,
}

impl SqlInsertBuilder {
	pub fn data(mut self, fields: Vec<Field>) -> Self {
		self.data = Some(fields);
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returning = Some(names.into_iter().map(|s| s.to_string()).collect());
		self
	}
}

impl SqlBuilder for SqlInsertBuilder {
	fn sql(&self) -> String {
		// SQL: INSERT INTO table_name (name1, ...) VALUES ($1, ...) RETURNING r1, ...;

		// SQL: INSERT INTO table_name
		let mut sql = String::from(format!("INSERT INTO \"{}\" ", self.table));

		// Note: empty data is a valid usecase, if the row has a all required field with default or auto gen.
		if let Some(fields) = &self.data {
			// SQL: (name1, name2, ...)
			sql.push_str(&format!("({}) ", sql_comma_names(fields)));

			// SQL: VALUES ($1, $2, ...)
			sql.push_str(&format!("VALUES ({}) ", sql_comma_params(fields)));
		}

		// SQL: RETURNING r1, r2, ...
		if let Some(returning) = &self.returning {
			let names = returning.join(", ");
			sql.push_str(&format!("returning {} ", names));
		}

		sql
	}

	fn vals(&self) -> Vec<Val> {
		match &self.data {
			Some(fields) => fields.iter().map(|f| f.1.clone()).collect(),
			None => Vec::new(),
		}
	}
}
