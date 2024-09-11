use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
	var: Variable,
	expr: Expr,
}

impl Assign {
	pub fn new(var: Variable, expr: Expr) -> Self {
		Self { var, expr }
	}
}
