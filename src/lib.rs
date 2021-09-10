mod delete;
mod insert;
mod select;
pub mod sqlx_exec;
mod update;
mod val;

pub use crate::delete::delete;
pub use crate::delete::delete_all;
pub use crate::insert::insert;
pub use crate::select::select;
pub use crate::update::update;
pub use crate::update::update_all;
pub use crate::val::Val;
pub use crate::val::ValType;

// region:    Field
#[derive(Clone)]
pub struct Field(pub String, pub Val);

pub trait GetFields {
	fn fields(&self) -> Vec<Field>;
}

impl<T: ValType> From<(&str, T)> for Field {
	fn from((name, value): (&str, T)) -> Self {
		Field(name.to_owned(), value.to_val())
	}
}
// endregion: Field

// region:    Common Types
#[derive(Clone)]
struct WhereItem {
	name: String,
	op: &'static str,
	val: Val,
}

impl<T: ValType> From<(&str, &'static str, T)> for WhereItem {
	fn from((name, op, value): (&str, &'static str, T)) -> Self {
		WhereItem {
			name: name.to_owned(),
			op,
			val: value.to_val(),
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

pub trait SqlBuilder {
	fn sql(&self) -> String;
	fn vals(&self) -> Vec<Val>;
}
// endregion: Common Types

// region:    property into helpers
fn into_and_wheres(and_wheres: Option<Vec<WhereItem>>, wheres: &[(&str, &'static str, impl ValType + Clone)]) -> Option<Vec<WhereItem>> {
	// Note: to_vec so that when it into_iter we do not get the reference of the tuple items
	let wheres = wheres.to_vec();
	let wheres: Vec<WhereItem> = wheres
		.into_iter()
		.map(|(name, op, val)| WhereItem {
			name: name.to_owned(),
			op,
			val: val.to_val(),
		})
		.collect();

	match and_wheres {
		Some(mut and_wheres) => {
			and_wheres.extend(wheres);
			Some(and_wheres)
		}
		None => Some(wheres),
	}
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
		.join(", ")
}

// SQL: "Id", "userName", ...
fn sql_returnings(returnings: &[String]) -> String {
	returnings.iter().map(|r| x_name(&r)).collect::<Vec<String>>().join(", ")
}
// endregion: Builder Utils

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
		let field = Field::from(("name1", "v2"));
		assert_eq!("name1", field.0);

		let field: Field = ("name1", &"v2".to_string()).into();
		assert_eq!("name1", field.0);
	}

	#[test]
	fn field_from_i64() {
		let field = Field::from(("name1", &"v2".to_string()));
		assert_eq!("name1", field.0);

		let field: Field = ("name1", &"v2".to_string()).into();
		assert_eq!("name1", field.0);
	}
}
