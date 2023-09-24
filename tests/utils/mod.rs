// #![allow(unused)]

use sqlb::Fields;

#[derive(Debug, Default, Fields)]
pub struct Todo {
	pub id: i64,
	pub title: String,

	#[field(name = "description")]
	pub desc: Option<String>,

	#[field(skip)]
	pub other: Option<String>,
}
