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

impl Display for Unary {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.op, self.expr)
	}
}
