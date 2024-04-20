use crate::model::{Stmt};

#[derive(Debug, PartialEq)]
pub struct Block {
	stmts: Vec<Stmt>,
}

impl Block {
	pub fn new(stmts: Vec<Stmt>) -> Self {
		Self { stmts }
	}

	pub fn stmts(&self) -> &Vec<Stmt> {
		&self.stmts
	}
}

impl std::fmt::Display for Block {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for stmt in &self.stmts {
			write!(f, "{}", stmt)?;
		}
		Ok(())
	}
}
