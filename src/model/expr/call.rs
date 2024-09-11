use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Call {
	pub callee: Box<Expr>,
	pub args: Vec<Expr>,
}

impl Call {
	pub fn new(callee: Expr, args: Vec<Expr>) -> Self {
		Self { callee: Box::new(callee), args }
	}
}
