use crate::model::{JsonExpression, Variable};

#[derive(Debug, PartialEq)]
pub struct Assign<'a> {
	var: Variable<'a>,
	expr: JsonExpression<'a>,
}

impl<'a> Assign<'a> {
	pub fn new(var: Variable<'a>, expr: JsonExpression<'a>) -> Self {
		Self { var, expr }
	}

	pub fn variable(&self) -> &Variable<'a> {
		&self.var
	}

	pub fn expression(&self) -> &JsonExpression<'a> {
		&self.expr
	}
}

impl std::fmt::Display for Assign<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} = {}", self.var, self.expr)
	}
}
