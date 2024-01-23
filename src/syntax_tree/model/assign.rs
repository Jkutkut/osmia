use crate::syntax_tree::model::{Expression, Variable};

#[derive(Debug, PartialEq)]
pub struct Assign<'a> {
	var: Variable<'a>,
	expr: Expression<'a>,
}

impl std::fmt::Display for Assign<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} = {}", self.var, self.expr)
	}
}
