mod utils;

use serial_test::serial;
use sqlb::sqlx_exec;
use std::error::Error;
use utils::init_db;

#[serial]
#[tokio::test]
async fn sb_transaction_ok_simple() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// -- Fixtures
	let test_title_1 = "test sb_transaction_ok_simple title 01";

	// -- Exec
	let fields = vec![("title", test_title_1).into()];
	let sb = sqlb::insert().table("todo").data(fields);
	let mut db_tx = db_pool.begin().await?;
	let row_affected = sqlx_exec::exec(&mut *db_tx, &sb).await?;

	// -- Check
	assert_eq!(1, row_affected, "row_affected");

	// NOTE: Assume if this works, the were tx could fail would be more on the sqlx side.

	Ok(())
}

#[serial]
#[tokio::test]
async fn sb_schema_ok_simple() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// -- Fixtures
	let test_title_1 = "test sb_schema_ok_simple title 01";

	// -- Exec
	let fields = vec![("title", test_title_1).into()];
	let sb = sqlb::insert().table("public.todo").data(fields);
	let row_affected = sqlx_exec::exec(&db_pool, &sb).await?;

	// -- Check
	assert_eq!(1, row_affected, "row_affected");

	// NOTE: Assume if this works, the were tx could fail would be more on the sqlx side.

	Ok(())
}
