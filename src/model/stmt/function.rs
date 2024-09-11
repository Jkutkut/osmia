use super::*;

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
