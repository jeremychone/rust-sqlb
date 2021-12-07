mod utils;

use sqlb::sqlx_exec::fetch_as_one;
use sqlb::{Fields, HasFields};
use utils::init_db;

#[tokio::test]
async fn macro_test_insert_full() -> Result<(), Box<dyn std::error::Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let fix_title = "macro_test_insert_full title".to_string();
	let fix_desc = "macro_test_insert_full desc".to_string();
	let todo = TodoCreate {
		title: fix_title.clone(),
		desc: Some(fix_desc.clone()),
	};

	// DO insert
	let sb = sqlb::insert().table("todo").data(todo.fields());
	let sb = sb.returning(&["id", "title", "desc"]);
	let (_id, title, desc) = fetch_as_one::<(i64, String, String), _, _>(&db_pool, &sb).await?;

	// CHECK title and desc
	assert_eq!(&fix_title, &title);
	assert_eq!(&fix_desc, &desc);

	Ok(())
}

#[tokio::test]
async fn macro_test_insert_partial() -> Result<(), Box<dyn std::error::Error>> {
	let db_pool = init_db().await?;

	// FIXTURES
	let fix_title = "macro_test_insert_partial title".to_string();
	let fix_desc: Option<String> = None;
	let todo = TodoCreate {
		title: fix_title.clone(),
		desc: fix_desc.clone(),
	};

	// DO insert
	let sb = sqlb::insert().table("todo").data(todo.fields());
	let sb = sb.returning(&["id", "title", "desc"]);
	let (_id, title, _) = fetch_as_one::<(i64, String, Option<String>), _, _>(&db_pool, &sb).await?;

	// CHECK title and desc
	assert_eq!(&fix_title, &title);
	assert_eq!(&fix_desc, &None);

	Ok(())
}

#[derive(Fields)]
struct TodoCreate {
	title: String,
	desc: Option<String>, // TODO: Need to handle Option
}
