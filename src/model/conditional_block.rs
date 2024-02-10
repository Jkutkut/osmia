use crate::model::{Expression, Stmt};

#[derive(Debug, PartialEq)]
pub struct ConditionalBlock<'a> {
	cond: Expression<'a>,
	body: Box<Stmt<'a>>,
}

impl<'a> ConditionalBlock<'a> {
	pub fn new(cond: Expression<'a>, body: Stmt<'a>) -> Self {
		Self { cond, body: Box::new(body) }
	}

	pub fn condition(&self) -> &Expression<'a> {
		&self.cond
	}

	pub fn body(&self) -> &Stmt<'a> {
		&self.body
	}
}

impl std::fmt::Display for ConditionalBlock<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "( {} ) {{\n{}}}", self.cond, self.body)
	}
}
