use super::into_and_wheres;
use crate::{sql_where_items, x_name, OrderItem, SqlBuilder, Val, ValType, WhereItem};

pub fn select(table: &str) -> SqlSelectBuilder {
	SqlSelectBuilder {
		table: table.to_string(),
		columns: None,
		and_wheres: None,
		order_bys: None,
	}
}

pub struct SqlSelectBuilder {
	table: String,
	columns: Option<Vec<String>>,
	// TODO: needs to support full condition (and/or)
	and_wheres: Option<Vec<WhereItem>>,
	order_bys: Option<Vec<OrderItem>>,
}

impl SqlSelectBuilder {
	pub fn columns(mut self, names: &[&str]) -> Self {
		self.columns = Some(names.into_iter().map(|s| s.to_string()).collect());
		self
	}

	pub fn and_where(mut self, wheres: &[(&str, &'static str, impl ValType + Clone)]) -> Self {
		self.and_wheres = into_and_wheres(self.and_wheres, wheres);
		self
	}

	pub fn order_bys(mut self, odrs: &[&str]) -> Self {
		self.order_bys = Some(odrs.to_vec().into_iter().map(|o| o.into()).collect());
		self
	}

	pub fn order_by(mut self, odr: &str) -> Self {
		self.order_bys = Some(vec![odr.into()]);
		self
	}
}

impl SqlBuilder for SqlSelectBuilder {
	fn sql(&self) -> String {
		// SELECT name1, name2 FROM table_name WHERE w1 < r1, w2 = r2

		// SQL: SELECT
		let mut sql = String::from("SELECT ");

		// SQL: name1, name2,
		// For now, if no column, will do a "*"
		match &self.columns {
			Some(columns) => {
				let names = columns.iter().map(|c| x_name(c)).collect::<Vec<String>>().join(", ");
				sql.push_str(&format!("{} ", names));
			}
			None => sql.push_str(&format!("{} ", "*")),
		};

		// SQL: FROM table_name
		sql.push_str(&format!("FROM {} ", x_name(&self.table)));

		// SQL: WHERE w1 < $1, ...
		if let Some(and_wheres) = &self.and_wheres {
			let sql_where = sql_where_items(&and_wheres, 1);
			sql.push_str(&format!("WHERE {} ", &sql_where));
		}

		// SQL: ORDER BY
		if let Some(order_bys) = &self.order_bys {
			let sql_order_bys = order_bys.iter().map::<String, _>(|o| o.into()).collect::<Vec<String>>().join(", ");
			sql.push_str(&format!("ORDER BY {} ", sql_order_bys))
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
