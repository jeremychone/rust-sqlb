use async_trait::async_trait;

pub use crate::delete::delete;
pub use crate::delete::delete_all;
pub use crate::delete::DeleteSqlBuilder;
pub use crate::insert::insert;
pub use crate::insert::InsertSqlBuilder;
pub use crate::select::select;
pub use crate::select::SelectSqlBuilder;
pub use crate::update::update;
pub use crate::update::update_all;
pub use crate::update::UpdateSqlBuilder;
pub use crate::val::SqlxBindable;
pub use sqlb_macros::Fields;
use sqlx::Executor;
use sqlx::FromRow;
use sqlx::Postgres;

pub struct Field<'a>(pub String, pub Box<dyn SqlxBindable + 'a + Send + Sync>);

impl<'a, T: 'a + SqlxBindable + Send + Sync> From<(&str, T)> for Field<'a> {
	fn from((name, value): (&str, T)) -> Self {
		Field(name.to_owned(), Box::new(value))
	}
}

pub trait HasFields {
	fn fields(&self) -> Vec<Field>;
}

// region:    Common Types
pub(crate) struct WhereItem<'a> {
	pub name: String,
	pub op: &'static str,
	pub val: Box<dyn SqlxBindable + 'a + Send + Sync>,
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
pub(crate) struct OrderItem {
	pub dir: OrderDir,
	pub name: String,
}

#[derive(Clone)]
pub(crate) enum OrderDir {
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

#[async_trait]
pub trait SqlBuilder<'a> {
	fn sql(&self) -> String;
	fn vals(&'a self) -> Box<dyn Iterator<Item = &Box<dyn SqlxBindable + 'a + Send + Sync>> + 'a + Send>;

	async fn fetch_one<'e, D, E>(&'a self, db_pool: E) -> Result<D, sqlx::Error>
	where
		E: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send;

	async fn fetch_all<'e, D, E>(&'a self, db_pool: E) -> Result<Vec<D>, sqlx::Error>
	where
		E: Executor<'e, Database = Postgres>,
		D: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + Send;

	async fn exec<'q, E>(&'a self, db_pool: E) -> Result<u64, sqlx::Error>
	where
		E: Executor<'q, Database = Postgres>;
}

pub trait Whereable<'a> {
	fn and_where_eq<T: 'a + SqlxBindable + Send + Sync>(self: Self, name: &str, val: T) -> Self;
	fn and_where<T: 'a + SqlxBindable + Send + Sync>(self: Self, name: &str, op: &'static str, val: T) -> Self;
}

// endregion: Common Types

// region:    property into helpers
pub(crate) fn add_to_where<'a, T: 'a + SqlxBindable + Send + Sync>(
	and_wheres: &mut Vec<WhereItem<'a>>,
	name: &str,
	op: &'static str,
	val: T,
) {
	// Note: to_vec so that when it into_iter we do not get the reference of the tuple items
	let wher = WhereItem {
		name: name.to_owned(),
		op,
		val: Box::new(val),
	};

	and_wheres.push(wher);
}

// Note: for now does not care about the base
pub(crate) fn into_returnings(_base: Option<Vec<String>>, names: &[&str]) -> Option<Vec<String>> {
	Some(names.into_iter().map(|s| s.to_string()).collect())
}
// endregion: property into helpers

// region:    Builder Utils
/// escape column name
/// TODO: needs to handle the . notation (i.e., quote each side of the dot)
pub(crate) fn x_name(name: &str) -> String {
	format!("\"{}\"", name)
}

// SQL: "name1", "name2", ...
pub(crate) fn sql_comma_names(fields: &[Field]) -> String {
	fields.iter().map(|Field(name, _)| x_name(name)).collect::<Vec<String>>().join(", ")
}

// SQL: $1, $2, $3, ...
pub(crate) fn sql_comma_params(fields: &[Field]) -> (i32, String) {
	let mut vals = String::new();
	let mut binding_idx = 1;

	for (idx, Field(_, val)) in fields.iter().enumerate() {
		if idx > 0 {
			vals.push_str(", ");
		};
		match val.raw() {
			None => {
				vals.push_str(&format!("${}", binding_idx));
				binding_idx += 1;
			}
			Some(raw) => vals.push_str(raw),
		};
	}
	(binding_idx, vals)
}

// If first array, idx_offset should be 1
// SQL: "name1" = &1, ...
pub(crate) fn sql_where_items(where_items: &[WhereItem], idx_start: usize) -> String {
	where_items
		.iter()
		.enumerate()
		.map(|(idx, WhereItem { name, op, .. })| format!("{} {} ${}", x_name(name), op, idx + idx_start))
		.collect::<Vec<String>>()
		.join(" AND ")
}

// SQL: "Id", "userName", ...
pub(crate) fn sql_returnings(returnings: &[String]) -> String {
	returnings.iter().map(|r| x_name(&r)).collect::<Vec<String>>().join(", ")
}
// endregion: Builder Utils
