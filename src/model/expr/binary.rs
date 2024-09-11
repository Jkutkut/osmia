use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
	pub left: Box<Expr>,
	pub operator: BinaryOp,
	pub right: Box<Expr>
}

impl Binary {
	pub fn new(
		left: Expr,
		operator: BinaryOp,
		right: Expr
	) -> Self {
		Self {
			left: Box::new(left),
			operator: operator,
			right: Box::new(right)
		}
	}
}
