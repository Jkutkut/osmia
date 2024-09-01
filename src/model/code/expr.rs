use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	Binary(Binary),
	Grouping(Grouping),

	Variable(Variable),

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

impl From<Binary> for Expr {
	fn from(b: Binary) -> Self {
		Self::Binary(b)
	}
}

impl From<Grouping> for Expr {
	fn from(g: Grouping) -> Self {
		Self::Grouping(g)
	}
}

impl From<Variable> for Expr {
	fn from(v: Variable) -> Self {
		Self::Variable(v)
	}
}
