use crate::model::{Stmt, ConditionalBlock};

#[derive(Debug, PartialEq)]
pub struct If {
	_if: ConditionalBlock,
	_elseifs: Option<Vec<ConditionalBlock>>,
	_else: Option<Box<Stmt>>,
}

impl If {
	pub fn new(
		if_block: ConditionalBlock,
		elseifs: Option<Vec<ConditionalBlock>>,
		else_block: Option<Stmt>
	) -> Self {
		let else_block = else_block.map(Box::new);
		Self {
			_if: if_block,
			_elseifs: elseifs,
			_else: else_block,
		}
	}

	pub fn if_block(&self) -> &ConditionalBlock {
		&self._if
	}

	pub fn elseifs(&self) -> &Option<Vec<ConditionalBlock>> {
		&self._elseifs
	}

	pub fn else_block(&self) -> &Option<Box<Stmt>> {
		&self._else
	}
}

impl std::fmt::Display for If {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "if {}", self._if)?;
		if let Some(elseifs) = &self._elseifs {
			for elseif in elseifs {
				write!(f, "elseif {}", elseif)?;
			}
		}
		if let Some(e) = &self._else {
			write!(f, "else {}", e)?;
		}
		Ok(())
	}
}
