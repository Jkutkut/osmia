use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
	pub expr: Box<Expr>
}

impl Grouping {
	pub fn new(expr: Expr) -> Self {
		Self { expr: Box::new(expr) }
	}
}
