use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCall {
	pub obj: Box<Expr>,
	pub call: Call,
}

impl MethodCall {
	pub fn new(obj: Expr, call: Call) -> Self {
		Self { obj: Box::new(obj), call }
	}
}
