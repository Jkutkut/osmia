use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
	arr: Vec<Expr>,
}

impl Array {
	pub fn new(arr: Vec<Expr>) -> Self {
		Self { arr }
	}
}
