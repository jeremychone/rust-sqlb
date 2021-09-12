//// NOTE: Very rudementary ValType/Val model. Exploration needed.

pub trait ValType {
	fn to_val(self) -> Val;
}

impl ValType for bool {
	fn to_val(self) -> Val {
		Val::BOOL(self)
	}
}

impl ValType for u32 {
	fn to_val(self) -> Val {
		Val::U32(self)
	}
}

impl ValType for i32 {
	fn to_val(self) -> Val {
		Val::I32(self)
	}
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
	BOOL(bool),
	U32(u32),
	I32(i32),
	I64(i64),
	STRING(String),
}
