use crate::core::WhereItem;
use crate::core::{add_to_where, into_returnings, sql_returnings, sql_where_items};
use crate::{SqlBuilder, SqlxBindable};

pub fn delete<'a>() -> SqlDeleteBuilder<'a> {
	SqlDeleteBuilder {
		guard_all: true,
		table: None,
		returnings: None,
		and_wheres: Vec::new(),
	}
}

pub fn delete_all<'a>() -> SqlDeleteBuilder<'a> {
	SqlDeleteBuilder {
		guard_all: false,
		table: None,
		returnings: None,
		and_wheres: Vec::new(),
	}
}

pub struct SqlDeleteBuilder<'a> {
	guard_all: bool,
	table: Option<String>,
	returnings: Option<Vec<String>>,
	and_wheres: Vec<WhereItem<'a>>,
}

impl<'a> SqlDeleteBuilder<'a> {
	pub fn table(mut self, table: &str) -> Self {
		self.table = Some(table.to_string());
		self
	}
	pub fn and_where<T: 'a + SqlxBindable + Send + Sync>(mut self, name: &str, op: &'static str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, op, val);
		self
	}

	pub fn and_where_eq<T: 'a + SqlxBindable + Send + Sync>(mut self, name: &str, val: T) -> Self {
		add_to_where(&mut self.and_wheres, name, "=", val);
		self
	}

	pub fn returning(mut self, names: &[&str]) -> Self {
		self.returnings = into_returnings(self.returnings, names);
		self
	}
}

impl<'a> SqlBuilder<'a> for SqlDeleteBuilder<'a> {
	fn sql(&self) -> String {
		// SQL: DELETE FROM table_name WHERE w1 = $1, ... RETURNING r1, r2, ..;

		// SQL: DELETE FROM table_name
		let mut sql = String::from("DELETE FROM ");

		if let Some(table) = &self.table {
			sql.push_str(&format!("\"{}\" ", table));
		}

		// SQL: WHERE w1 < $1, ...
		if self.and_wheres.len() > 0 {
			let sql_where = sql_where_items(&self.and_wheres, 1);
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

	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send> {
		let iter = self.and_wheres.iter().map(|wi| &wi.val);
		Box::new(iter)
	}
}
