use crate::model::{Expression, Stmt};

#[derive(Debug, PartialEq)]
pub struct ConditionalBlock {
	cond: Expression,
	body: Box<Stmt>,
}

impl ConditionalBlock {
	pub fn new(cond: Expression, body: Stmt) -> Self {
		Self { cond, body: Box::new(body) }
	}

	pub fn condition(&self) -> &Expression {
		&self.cond
	}

	pub fn body(&self) -> &Stmt {
		&self.body
	}
}

impl std::fmt::Display for ConditionalBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "( {} ) {{\n{}}}", self.cond, self.body)
	}
}
