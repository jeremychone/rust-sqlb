//! Mostly for dev sqlx API validation.

#![allow(unused)] // For early development.

mod utils;

use crate::utils::init_db;
use serial_test::serial;
use std::error::Error;

// #[serial]
// #[tokio::test]
async fn test_sqlx_insert_todo() -> Result<(), Box<dyn Error>> {
	let db = init_db().await?;
	let fx_title = "test_sqlx_insert_todo - title".to_string();
	let fx_desc: Option<String> = None;

	let (id,) = sqlx::query_as::<_, (i64,)>(r#"INSERT INTO todo (title, "desc") values ($1, $2) returning id"#)
		.bind(fx_title)
		.bind(fx_desc)
		.fetch_one(&db)
		.await?;

	Ok(())
}
