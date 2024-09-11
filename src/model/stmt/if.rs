use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct If {
	_if: ConditionalStmt,
	_elseifs: Option<Vec<ConditionalStmt>>,
	_else: Option<Box<Stmt>>,
}

impl If {
	pub fn new(
		if_block: ConditionalStmt,
		elseifs: Option<Vec<ConditionalStmt>>,
		else_block: Option<Stmt>
	) -> Self {
		let else_block = else_block.map(Box::new);
		Self {
			_if: if_block,
			_elseifs: elseifs,
			_else: else_block,
		}
	}
}
