use super::*;

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
