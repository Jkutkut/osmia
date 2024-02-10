use crate::model::{Stmt};

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
	stmts: Vec<Stmt<'a>>,
}

impl<'a> Block<'a> {
	pub fn new(stmts: Vec<Stmt<'a>>) -> Self {
		Self { stmts }
	}

	pub fn stmts(&self) -> &Vec<Stmt<'a>> {
		&self.stmts
	}
}

impl std::fmt::Display for Block<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for stmt in &self.stmts {
			write!(f, "{}", stmt)?;
		}
		Ok(())
	}
}
