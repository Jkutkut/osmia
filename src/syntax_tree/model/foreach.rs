use crate::syntax_tree::model::{Variable, Block};

#[derive(Debug, PartialEq)]
pub struct Foreach<'a> {
	var: Variable<'a>,
	list: Variable<'a>,
	body: Block<'a>,
}

impl std::fmt::Display for Foreach<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "foreach ( {} in {} ) {{\n{}}}", self.var, self.list, self.body)
	}
}
