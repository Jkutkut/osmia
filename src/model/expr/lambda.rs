use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
	params: Vec<FunctionParam>,
	body: Box<Expr>,
}

impl Lambda {
	pub fn new(params: Vec<FunctionParam>, body: Expr) -> Self {
		Self { params, body: Box::new(body) }
	}
}
