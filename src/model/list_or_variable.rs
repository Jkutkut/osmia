use crate::model::{Variable, JsonExpression};

#[derive(Debug, PartialEq)]
pub enum ListOrVariable {
	List(JsonExpression),
	Variable(Variable),
}

impl std::fmt::Display for ListOrVariable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ListOrVariable::List(ref list) => write!(f, "{}", list),
			ListOrVariable::Variable(ref variable) => write!(f, "{}", variable),
		}
	}
}
