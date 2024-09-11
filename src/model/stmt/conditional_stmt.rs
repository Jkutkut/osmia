use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalStmt {
	cond: Expr,
	body: Box<Stmt>,
}

impl ConditionalStmt {
	pub fn new(cond: Expr, body: Stmt) -> Self {
		Self { cond, body: Box::new(body) }
	}

	pub fn condition(&self) -> &Expr {
		&self.cond
	}

	pub fn body(&self) -> &Stmt {
		&self.body
	}
}
