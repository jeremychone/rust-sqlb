pub mod sqlx_exec;

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
	#[allow(unused)]
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

// region:    Insert Builder
// INSERT INTO table_name (column1, ...) VALUES ($1, ...) RETURNING r1, r2, ;
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

// endregion: Insert Builder

// region:    Select Builder =========

// SELECT name1, name2 FROM table_name WHERE w1 < r1, w2 = r2
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

	#[allow(unused)]
	pub fn and_where(mut self, wheres: &[(&str, &str, Val)]) -> Self {
		// Note: to_vec so that when it into_iter we do not get the reference of the tuple items
		let wheres = wheres.to_vec();
		let wheres: Vec<WhereItem> = wheres
			.into_iter()
			.map(|(name, op, val)| WhereItem {
				name: name.to_owned(),
				op: op.to_owned(),
				val,
			})
			.collect();

		self.and_wheres = match self.and_wheres {
			Some(mut and_wheres) => {
				and_wheres.extend(wheres);
				Some(and_wheres)
			}
			None => Some(wheres),
		};

		self
	}

	#[allow(unused)]
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
			let sql_where = and_wheres
				.iter()
				.enumerate()
				.map(|(idx, WhereItem { name, op, .. })| format!("{} {} ${}", x_name(name), op, idx + 1))
				.collect::<Vec<String>>()
				.join(", ");
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
// endregion: Select Builder =========

// region:    Update Builder
// UPDATE table_name SET column1 = value1, ... WHERE condition;
// endregion: Update Builder

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

#[cfg(test)]
mod tests {
	use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
	use std::error::Error;

	use crate::{Field, GetFields};

	// region:    Test Types
	#[derive(sqlx::FromRow)]
	pub struct Todo {
		pub id: i64,
		pub title: String,
	}

	pub struct TodoPatch {
		pub title: Option<String>,
	}

	impl GetFields for TodoPatch {
		fn fields(&self) -> Vec<Field> {
			let mut fields = Vec::new();
			if let Some(title) = &self.title {
				fields.push(Field::from_string("title", title));
			}
			fields
		}
	}

	// endregion: Test Types

	#[test]
	fn it_works() {
		println!("->> HELLO {}", 123);
		assert_eq!(2 + 2, 4);
	}

	/// Simple and dirty very first test. Should be moved to integrated test.
	#[tokio::test]
	async fn integrated_insert_and_select() -> Result<(), Box<dyn Error>> {
		let db_pool = init_db().await?;

		let test_title = "test - title 01";

		let patch_data = TodoPatch {
			title: Some(test_title.to_string()),
		};

		// test insert with returning
		let sb = super::insert("todo").data(patch_data.fields());
		let sb = sb.returning(&["id", "title"]);
		let (_id, title) = super::sqlx_exec::fetch_as_one::<(i64, String), _>(&db_pool, &sb).await?;
		assert_eq!(test_title, title);

		// test select
		let sb = super::select("todo").columns(&["id", "title"]).order_by("!id");
		let todos: Vec<Todo> = super::sqlx_exec::fetch_as_all(&db_pool, &sb).await?;
		assert_eq!(1, todos.len());
		assert_eq!(test_title, todos[0].title);

		Ok(())
	}

	async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect("postgres://postgres:welcome@localhost/postgres")
			.await?;

		// Create todo table
		sqlx::query("DROP TABLE IF EXISTS todo").execute(&pool).await?;
		sqlx::query(
			r#"
CREATE TABLE IF NOT EXISTS todo (
  id bigserial,
  title text
);"#,
		)
		.execute(&pool)
		.await?;

		// Create project table
		sqlx::query("DROP TABLE IF EXISTS projects").execute(&pool).await?;
		sqlx::query(
			r#"
CREATE TABLE IF NOT EXISTS project (
  id bigserial,
  name text
);"#,
		)
		.execute(&pool)
		.await?;

		Ok(pool)
	}
}
