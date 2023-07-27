/// Escape table name.
/// - Surround with `"` if simple table name.
/// - If the text contains a . symbol, ensure to surround each part.
///
/// TODO: needs to handle the . notation (i.e., quote each side of the dot)
pub(crate) fn x_table_name(name: &str) -> String {
	if name.contains('.') {
		name.split('.')
			.map(|part| format!("\"{}\"", part))
			.collect::<Vec<String>>()
			.join(".")
	} else {
		format!("\"{}\"", name)
	}
}

/// Escape column name.
/// - Surround with `"` if simple column name.
/// - Leave column name as is if special character `(` (might need to add more)
///   (this allows function call like `count(*)`)
/// - If the text contains a . symbol, ensure to surround each part.
///
pub(crate) fn x_column_name(name: &str) -> String {
	if name.contains('(') {
		name.to_string()
	} else if name.contains('.') {
		name.split('.')
			.map(|part| format!("\"{}\"", part))
			.collect::<Vec<String>>()
			.join(".")
	} else {
		format!("\"{}\"", name)
	}
}
