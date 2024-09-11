use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
	op: UnaryOp,
	expr: Box<Expr>,
}

impl Unary {
	pub fn new(op: UnaryOp, expr: Expr) -> Self {
		Self { op, expr: Box::new(expr) }
	}
}
