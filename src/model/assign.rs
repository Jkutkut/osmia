use crate::model::{JsonExpression, Variable};

#[derive(Debug, PartialEq)]
pub struct Assign {
	var: Variable,
	expr: JsonExpression,
}

impl Assign {
	pub fn new(var: Variable, expr: JsonExpression) -> Self {
		Self { var, expr }
	}

	pub fn variable(&self) -> &Variable {
		&self.var
	}

	pub fn expression(&self) -> &JsonExpression {
		&self.expr
	}
}

impl std::fmt::Display for Assign {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} = {}", self.var, self.expr)
	}
}
