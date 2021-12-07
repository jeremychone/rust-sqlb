mod utils;

use sqlb::sqlx_exec;
use std::error::Error;
use utils::init_db;

#[tokio::test]
async fn sb_transaction_simple() -> Result<(), Box<dyn Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let test_title_1 = "test title 01";

	// DO insert
	let fields = vec![("title", test_title_1).into()];
	let sb = sqlb::insert().table("todo").data(fields);
	let mut db_tx = db_pool.begin().await?;
	let row_affected = sqlx_exec::exec(&mut db_tx, &sb).await?;

	// CHECK row_affected
	assert_eq!(1, row_affected, "row_affected");

	// NOTE: Assume if this works, the were tx could fail would be more on the sqlx side.

	Ok(())
}
