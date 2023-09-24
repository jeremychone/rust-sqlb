use crate::{Error, Result};
use sea_query::ValueType;
use sea_query::{DynIden, Iden, SimpleExpr, Value};

// region:    --- Field
#[derive(Debug, Clone)]
pub struct Field {
	pub name: DynIden,
	pub value: SimpleExpr,
}

impl Field {
	pub fn sea_value(&self) -> Option<&Value> {
		if let SimpleExpr::Value(value) = &self.value {
			// value.unwrap()
			Some(value)
		} else {
			None
		}
	}

	pub fn value_into<T>(self) -> Result<T>
	where
		T: ValueType,
	{
		let SimpleExpr::Value(value) = self.value else {
			return Err(Error::FieldValueNotSeaValue);
		};

		let field_name = self.name;
		T::try_from(value).map_err(|_| Error::FieldValueIntoTypeError {
			field_name: field_name.to_string(),
		})
	}
}

impl Field {
	pub fn new(name: DynIden, value: SimpleExpr) -> Self {
		Field { name, value }
	}
}

// endregion: --- Field

// region:    --- Fields
#[derive(Debug, Clone)]
pub struct Fields(Vec<Field>);

// Constructor
impl Fields {
	pub fn new(fields: Vec<Field>) -> Self {
		Fields(fields)
	}
}

// Api
impl Fields {
	pub fn push(&mut self, field: Field) {
		self.0.push(field);
	}

	pub fn into_vec(self) -> Vec<Field> {
		self.0
	}

	/// returns a tuble: (Vec_of_column_idens, Vec_of_value_exprs)
	pub fn unzip(self) -> (Vec<DynIden>, Vec<SimpleExpr>) {
		self.0.into_iter().map(|f| (f.name, f.value)).unzip()
	}

	/// returns Iterator of (column_iden, value_expr)
	pub fn zip(self) -> impl Iterator<Item = (DynIden, SimpleExpr)> {
		self.0.into_iter().map(|f| (f.name, f.value))
	}
}

impl IntoIterator for Fields {
	type Item = Field;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

// endregion: --- Fields

// region:    --- HasFields
pub trait HasFields {
	/// Consume and returns the `Field(name, value)` where the value is a not none `SqlxBindable`.
	fn not_none_fields(self) -> Fields;

	/// Consume and returns the `Field(name, value)` where the value is a `SqlxBindable`.
	fn all_fields(self) -> Fields;

	/// Return the array of all field names this struct has.
	fn field_names() -> &'static [&'static str];

	fn field_idens() -> Vec<sea_query::SeaRc<dyn sea_query::Iden>>;
}
// endregion: --- HasFields

// region:    --- Private Utils
#[derive(Debug)]
pub struct SIden(pub &'static str);
impl Iden for SIden {
	fn unquoted(&self, s: &mut dyn std::fmt::Write) {
		s.write_str(self.0).expect("SCol write_str fatal error");
	}
}
// endregion: --- Private Utils
