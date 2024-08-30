use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
	Block(Block),
	Raw(String),
	Expr(Expr),
}

impl Stmt {
	pub fn new_raw(s: &str) -> Self {
		Self::Raw(s.to_string())
	}
}

impl From<Block> for Stmt {
	fn from(block: Block) -> Self {
		Self::Block(block)
	}
}

impl From<Expr> for Stmt {
	fn from(expr: Expr) -> Self {
		Self::Expr(expr)
	}
}
