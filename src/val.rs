//// NOTE: Very rudementary ValType/Val model. Exploration needed.

pub trait ValType {
	fn to_val(self) -> Val;
}

impl ValType for i64 {
	fn to_val(self) -> Val {
		Val::I64(self)
	}
}

impl ValType for String {
	fn to_val(self) -> Val {
		Val::STRING(self)
	}
}

impl ValType for &String {
	fn to_val(self) -> Val {
		Val::STRING(self.to_owned())
	}
}

impl ValType for &str {
	fn to_val(self) -> Val {
		Val::STRING(self.to_owned())
	}
}

#[derive(Clone)]
pub enum Val {
	I64(i64),
	STRING(String),
}
