use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct For {
	var: Variable,
	iterable: Expr,
	body: Box<Stmt>,
}

impl For {
	pub fn new(var: Variable, iterable: Expr, body: Stmt) -> Self {
		Self { var, iterable, body: Box::new(body) }
	}

	pub fn variable(&self) -> &Variable {
		&self.var
	}

	pub fn iterable(&self) -> &Expr {
		&self.iterable
	}

	pub fn body(&self) -> &Stmt {
		&self.body
	}
}
