use super::*;

pub enum Stmt {
	Block(Block),
}

impl From<Block> for Stmt {
	fn from(block: Block) -> Self {
		Self::Block(block)
	}
}
