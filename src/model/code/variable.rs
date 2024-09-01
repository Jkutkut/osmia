use crate::model::ctx::JsonTreeKey;
use crate::model::code::Expr;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
	var: Vec<JsonTreeKeyExpression>,
}

impl Variable {
	pub fn from_vec(var: Vec<JsonTreeKeyExpression>) -> Self {
		Self { var }
	}

	pub fn push(&mut self, key: JsonTreeKeyExpression) {
		self.var.push(key)
	}

	pub fn vec(&self) -> &Vec<JsonTreeKeyExpression> {
		&self.var
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonTreeKeyExpression {
	JsonTreeKey(JsonTreeKey<String>),
	Expr(Expr),
}

impl From<&str> for JsonTreeKey<String> {
	fn from(s: &str) -> Self {
		JsonTreeKey::Key(s.into())
	}
}

impl From<JsonTreeKey<String>> for JsonTreeKeyExpression {
	fn from(key: JsonTreeKey<String>) -> Self {
		JsonTreeKeyExpression::JsonTreeKey(key)
	}
}

impl From<Expr> for JsonTreeKeyExpression {
	fn from(expr: Expr) -> Self {
		JsonTreeKeyExpression::Expr(expr)
	}
}
