mod insert;
mod select;
pub mod sqlx_exec;

pub use crate::insert::insert;
pub use crate::select::select;

// region:    Common Types
#[derive(Clone)]
pub struct Field(pub String, pub Val);

pub trait GetFields {
	fn fields(&self) -> Vec<Field>;
}

impl Field {
	pub fn from_string(name: &str, val: &str) -> Field {
		Field(name.to_owned(), Val::STRING(val.to_owned()))
	}
}

//// NOTE: Very rudementary Value enum. To refactor before beta.
#[derive(Clone)]
pub enum Val {
	I64(i64),
	STRING(String),
}

#[derive(Clone)]
struct WhereItem {
	name: String,
	op: String,
	val: Val,
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

pub trait SqlBuilder {
	fn sql(&self) -> String;
	fn vals(&self) -> Vec<Val>;
}
// endregion: Common Types

// region:    Builder Utils

/// escape column name
/// TODO, needs to handle the . notation (i.e., quote each side of the dot)
fn x_name(name: &str) -> String {
	format!("\"{}\"", name)
}

fn sql_comma_names(fields: &[Field]) -> String {
	fields.iter().map(|Field(name, _)| x_name(name)).collect::<Vec<String>>().join(", ")
}

fn sql_comma_params(fields: &[Field]) -> String {
	(0..fields.len()).into_iter().map(|i| format!("${}", i + 1)).collect::<Vec<String>>().join(", ")
}
// endregion: Builder Utils
