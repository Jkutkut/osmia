use super::*;

pub struct Block {
	pub stmts: Vec<Stmt>,
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
}

impl From<Block> for Vec<Stmt> {
	fn from(block: Block) -> Self {
		block.stmts
	}
}
