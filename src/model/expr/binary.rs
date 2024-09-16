use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
	left: Box<Expr>,
	operator: BinaryOp,
	right: Box<Expr>
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

	pub fn left(&self) -> &Expr {
		&self.left
	}

	pub fn operator(&self) -> &BinaryOp {
		&self.operator
	}

	pub fn right(&self) -> &Expr {
		&self.right
	}
}
