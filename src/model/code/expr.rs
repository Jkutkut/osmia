#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	Float(f64),
	Int(i64),
	Str(String),
	Bool(bool),
	Null,
}

impl Expr {
	pub fn new_str(s: &str) -> Self {
		Self::Str(s.to_string())
	}
}
