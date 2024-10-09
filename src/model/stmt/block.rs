use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
	stmts: Vec<Stmt>,
}

impl Block {
	pub fn new() -> Self {
		Self { stmts: vec![] }
	}

	pub fn push(&mut self, stmt: Stmt) {
		self.stmts.push(stmt);
	}

	pub fn len(&self) -> usize {
		self.stmts.len()
	}

	pub fn stmts(&self) -> &Vec<Stmt> {
		&self.stmts
	}
}

impl From<Block> for Vec<Stmt> {
	fn from(block: Block) -> Self {
		block.stmts
	}
}

impl From<Vec<Stmt>> for Block {
	fn from(stmts: Vec<Stmt>) -> Self {
		Self { stmts }
	}
}
