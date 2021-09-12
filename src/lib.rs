#![allow(unused)] // silence unused warnings while exploring (to comment out)

mod delete;
mod insert;
mod select;
pub mod sqlx_exec;
mod update;
mod val;

use sqlx::Encode;
use sqlx::Postgres;

pub use crate::delete::delete;
pub use crate::delete::delete_all;
pub use crate::delete::SqlDeleteBuilder;
pub use crate::insert::insert;
pub use crate::insert::SqlInsertBuilder;
pub use crate::select::select;
pub use crate::select::SqlSelectBuilder;
pub use crate::update::update;
pub use crate::update::update_all;
pub use crate::update::SqlUpdateBuilder;
pub use crate::val::Field;
pub use crate::val::GetFields;
pub use crate::val::SqlxBindable;

// region:    Common Types
struct WhereItem<'a> {
	name: String,
	op: &'static str,
	val: Box<dyn SqlxBindable + 'a + Send + Sync>,
}

impl<'a, T: 'a + SqlxBindable + Send + Sync> From<(&str, &'static str, T)> for WhereItem<'a> {
	fn from((name, op, value): (&str, &'static str, T)) -> Self {
		WhereItem {
			name: name.to_owned(),
			op,
			val: Box::new(value),
		}
	}
}

#[derive(Clone)]
struct OrderItem {
	dir: OrderDir,
	name: String,
}
#[derive(Clone)]
enum OrderDir {
	ASC,
	DESC,
}

impl From<&str> for OrderItem {
	fn from(v: &str) -> Self {
		if v.starts_with("!") {
			OrderItem {
				dir: OrderDir::DESC,
				name: x_name(&v[1..]),
			}
		} else {
			OrderItem {
				dir: OrderDir::ASC,
				name: x_name(v),
			}
		}
	}
}

impl From<&OrderItem> for String {
	fn from(odr: &OrderItem) -> Self {
		match odr.dir {
			OrderDir::ASC => format!("{}", odr.name),
			OrderDir::DESC => format!("{} {}", odr.name, "DESC"),
		}
	}
}

pub trait SqlBuilder<'a> {
	fn sql(&self) -> String;
	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send>;
}

// endregion: Common Types

// region:    property into helpers
fn add_to_where<'a, T: 'a + SqlxBindable + Send + Sync>(and_wheres: &mut Vec<WhereItem<'a>>, name: &str, op: &'static str, val: T) {
	// Note: to_vec so that when it into_iter we do not get the reference of the tuple items
	let wher = WhereItem {
		name: name.to_owned(),
		op,
		val: Box::new(val),
	};

	and_wheres.push(wher);
}

// Note: for now does not care about the base
fn into_returnings(_base: Option<Vec<String>>, names: &[&str]) -> Option<Vec<String>> {
	Some(names.into_iter().map(|s| s.to_string()).collect())
}
// endregion: property into helpers

// region:    Builder Utils
/// escape column name
/// TODO, needs to handle the . notation (i.e., quote each side of the dot)
fn x_name(name: &str) -> String {
	format!("\"{}\"", name)
}

// SQL: "name1", "name2", ...
fn sql_comma_names(fields: &[Field]) -> String {
	fields.iter().map(|Field(name, _)| x_name(name)).collect::<Vec<String>>().join(", ")
}

// SQL: $1, $2, $3, ...
fn sql_comma_params(fields: &[Field]) -> String {
	(0..fields.len()).into_iter().map(|i| format!("${}", i + 1)).collect::<Vec<String>>().join(", ")
}

// If first array, idx_offset should be 1
// SQL: "name1" = &1, ...
fn sql_where_items(where_items: &[WhereItem], idx_start: usize) -> String {
	where_items
		.iter()
		.enumerate()
		.map(|(idx, WhereItem { name, op, .. })| format!("{} {} ${}", x_name(name), op, idx + idx_start))
		.collect::<Vec<String>>()
		.join(" AND ")
}

// SQL: "Id", "userName", ...
fn sql_returnings(returnings: &[String]) -> String {
	returnings.iter().map(|r| x_name(&r)).collect::<Vec<String>>().join(", ")
}
// endregion: Builder Utils
