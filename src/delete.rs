use crate::{into_and_wheres, into_returnings, sql_returnings, sql_where_items, SqlBuilder, Val, WhereItem};

pub fn delete(table: &str) -> SqlDeleteBuilder {
	SqlDeleteBuilder {
		guard_all: true,
		table: table.to_string(),
		returnings: None,
		and_wheres: None,
	}
}

pub fn delete_all(table: &str) -> SqlDeleteBuilder {
	SqlDeleteBuilder {
		guard_all: false,
		table: table.to_string(),
		returnings: None,
		and_wheres: None,
	}
}

#[derive(Clone)]
pub struct SqlDeleteBuilder {
	guard_all: bool,
	table: String,
	returnings: Option<Vec<String>>,
	and_wheres: Option<Vec<WhereItem>>,
}

impl SqlDeleteBuilder {
	pub fn and_where(mut self, wheres: &[(&str, &str, Val)]) -> Self {
		self.and_wheres = into_and_wheres(self.and_wheres, wheres);
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returnings = into_returnings(self.returnings, names);
		self
	}
}

impl SqlBuilder for SqlDeleteBuilder {
	fn sql(&self) -> String {
		// SQL: DELETE FROM table_name WHERE w1 = $1, ... RETURNING r1, r2, ..;

		// SQL: DELETE FROM table_name
		let mut sql = String::from(format!("DELETE FROM \"{}\" ", self.table));

		// SQL: WHERE w1 < $1, ...
		if let Some(and_wheres) = &self.and_wheres {
			let sql_where = sql_where_items(&and_wheres, 1);
			sql.push_str(&format!("WHERE {} ", &sql_where));
		} else if self.guard_all {
			// For now panic, will return error later
			panic!("FATAL - Trying to call a delete without any where clause. If needed, use sqlb::delete_all(table_name). ")
		}

		// SQL: RETURNING "r1", "r2", ...
		if let Some(returnings) = &self.returnings {
			sql.push_str(&format!("RETURNING {} ", sql_returnings(returnings)));
		}

		sql
	}

	fn vals(&self) -> Vec<Val> {
		match &self.and_wheres {
			Some(and_wheres) => and_wheres.iter().map(|w| w.val.clone()).collect(),
			None => Vec::new(),
		}
	}
}
