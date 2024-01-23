use crate::syntax_tree::model::{Expression, Block};

#[derive(Debug, PartialEq)]
pub struct ConditionalBlock<'a> {
	cond: Expression<'a>,
	body: Block<'a>,
}

impl std::fmt::Display for ConditionalBlock<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "( {} ) {{\n{}}}", self.cond, self.body)
	}
}
