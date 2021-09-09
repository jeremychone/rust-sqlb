//// NOTE: Very rudementary Value enum. To refactor before beta.
#[derive(Clone)]
pub enum Val {
	I64(i64),
	STRING(String),
}

impl From<i64> for Val {
	fn from(v: i64) -> Self {
		Val::I64(v)
	}
}

// TODO: need to see smarter way to handle String, &String, &str
impl From<String> for Val {
	fn from(v: String) -> Self {
		// Note: for now, we do a move, and see the impact on the API
		Val::STRING(v)
	}
}

// TODO: need to see smarter way to handle String, &String, &str
impl From<&String> for Val {
	fn from(v: &String) -> Self {
		// Note: for now, we do a move, and see the impact on the API
		Val::STRING(v.to_string())
	}
}

// TODO: need to see smarter way to handle String, &String, &str
impl From<&str> for Val {
	fn from(v: &str) -> Self {
		Val::STRING(v.to_string())
	}
}
