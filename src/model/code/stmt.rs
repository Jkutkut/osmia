use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
	Block(Block),
	Raw(String),
}

impl From<Block> for Stmt {
	fn from(block: Block) -> Self {
		Self::Block(block)
	}
}
