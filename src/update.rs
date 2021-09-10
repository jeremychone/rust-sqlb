use crate::{Field, SqlBuilder, Val, ValType, WhereItem, into_and_wheres, into_returnings, sql_returnings, sql_where_items, x_name};

pub fn update(table: &str) -> SqlUpdateBuilder {
	SqlUpdateBuilder {
		guard_all: true,
		table: table.to_string(),
		data: None,
		returnings: None,
		and_wheres: None,
	}
}

pub fn update_all(table: &str) -> SqlUpdateBuilder {
	SqlUpdateBuilder {
		guard_all: false,
		table: table.to_string(),
		data: None,
		returnings: None,
		and_wheres: None,
	}
}

#[derive(Clone)]
pub struct SqlUpdateBuilder {
	guard_all: bool,
	table: String,
	data: Option<Vec<Field>>,
	returnings: Option<Vec<String>>,
	and_wheres: Option<Vec<WhereItem>>,
}

impl SqlUpdateBuilder {
	pub fn data(mut self, fields: Vec<Field>) -> Self {
		self.data = Some(fields);
		self
	}

	pub fn and_where(mut self, wheres: &[(&str, &str, impl ValType + Clone)]) -> Self {
		self.and_wheres = into_and_wheres(self.and_wheres, wheres);
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returnings = into_returnings(self.returnings, names);
		self
	}
}

impl SqlBuilder for SqlUpdateBuilder {
	fn sql(&self) -> String {
		// SQL: UPDATE table_name SET column1 = $1, ... WHERE w1 = $2, w2 = $3 returning r1, r2;

		// SQL: UPDATE table_name SET
		let mut sql = String::from(format!("UPDATE \"{}\" SET ", self.table));

		// The index for the params since we ahve two array
		let mut idx_start = 1;

		// TODO: Handle the case of empty data. Should we change this signature to return a Result ?
		//       For now, just ignore this case, will fail at sql exec time
		// SQL: column1 = $1, ...
		if let Some(fields) = &self.data {
			let sql_set = fields
				.iter()
				.enumerate()
				.map(|(idx, f)| format!("{} = ${}", x_name(&f.0), idx + idx_start))
				.collect::<Vec<String>>()
				.join(", ");
			sql.push_str(&format!("{} ", sql_set));
			// update idx_start for next eventual parameters
			idx_start = idx_start + fields.len();
		}

		// SQL: WHERE w1 < $1, ...
		if let Some(and_wheres) = &self.and_wheres {
			let sql_where = sql_where_items(&and_wheres, idx_start);
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

	fn vals(&self) -> Vec<Val> {
		let mut base = match &self.data {
			Some(fields) => fields.iter().map(|f| f.1.clone()).collect(),
			None => Vec::new(),
		};
		if let Some(where_items) = &self.and_wheres {
			base.extend(where_items.iter().map(|wi| wi.val.clone()));
		}
		base
	}
}
