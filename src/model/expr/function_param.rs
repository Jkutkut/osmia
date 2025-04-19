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

	pub fn name(&self) -> &JsonTreeKey<String> {
		match self {
			Self::Param(name, _) => name,
			Self::Spread(name) => name,
		}
	}
}

impl std::fmt::Display for FunctionParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Param(name, None) => write!(f, "{}", name),
			Self::Param(name, Some(expr)) => write!(f, "{} = {}", name, expr),
			Self::Spread(name) => write!(f, "...{}", name),
		}
	}
}
