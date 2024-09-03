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

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionParam {
	Param(JsonTreeKey<String>, Option<Expr>),
	Spread(JsonTreeKey<String>),
}

impl FunctionParam {
	pub fn new(name: JsonTreeKey<String>, default: Option<Expr>) -> Self {
		Self::Param(name, default)
	}

	pub fn new_spread(name: JsonTreeKey<String>) -> Self {
		Self::Spread(name)
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
	name: JsonTreeKey<String>,
	params: Vec<FunctionParam>,
	body: Box<Stmt>,
}

impl Function {
	pub fn new(name: JsonTreeKey<String>, params: Vec<FunctionParam>, body: Stmt) -> Self {
		Self { name, params, body: Box::new(body) }
	}
}
