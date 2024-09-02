use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
	expr: Option<Expr>
}

impl Return {
	pub fn new(expr: Option<Expr>) -> Self {
		Self { expr }
	}
}
