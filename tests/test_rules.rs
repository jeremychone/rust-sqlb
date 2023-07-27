mod utils;

use serial_test::serial;
use sqlb::{bindable, Field};
use std::error::Error;
use utils::init_db;

// region:    Custom Type (enum)
#[derive(Eq, PartialEq, Hash, sqlx::Type, Debug, Clone)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
	New,
	Open,
	Done,
}

// OR
bindable!(TodoStatus);

// NOTE: This test is just about passing the compile time.
#[serial]
#[tokio::test]
async fn test_rules_custom_enum() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// CHECK that the SqlxBindable is implemented for TodoStatus
	let title_1 = "test - test_rules_custom_enum title".to_string();
	let _data: Vec<Field> = vec![("title", title_1).into(), ("status", TodoStatus::Open).into()];

	// CHECK that the TodoStatus has the appropriate types to pass sqlx binding (no sqlb::SqlxBindable at this stage)
	let query = sqlx::query::<sqlx::Postgres>("INSERT INTO todo (title, status) VALUES ($1, $2)");
	let query = query.bind("test sb_enum_insert 01");
	let query = query.bind(TodoStatus::Done);
	let _r = query.execute(&db_pool).await?.rows_affected();

	Ok(())
}
