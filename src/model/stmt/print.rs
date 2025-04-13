use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Print {
	expr: Expr
}

impl Print {
	pub fn new(expr: Expr) -> Self {
		Self { expr }
	}

	pub fn expr(&self) -> &Expr {
		&self.expr
	}
}
