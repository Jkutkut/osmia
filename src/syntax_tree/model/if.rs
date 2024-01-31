use crate::syntax_tree::model::{Stmt, ConditionalBlock};

#[derive(Debug, PartialEq)]
pub struct If<'a> {
	_if: ConditionalBlock<'a>,
	_elseifs: Option<Vec<ConditionalBlock<'a>>>,
	_else: Option<Box<Stmt<'a>>>,
}

impl<'a> If<'a> {
	pub fn new(
		if_block: ConditionalBlock<'a>,
		elseifs: Option<Vec<ConditionalBlock<'a>>>,
		else_block: Option<Stmt<'a>>
	) -> Self {
		let else_block = else_block.map(Box::new);
		Self {
			_if: if_block,
			_elseifs: elseifs,
			_else: else_block,
		}
	}

	pub fn if_block(&self) -> &ConditionalBlock<'a> {
		&self._if
	}

	pub fn elseifs(&self) -> &Option<Vec<ConditionalBlock<'a>>> {
		&self._elseifs
	}

	pub fn else_block(&self) -> &Option<Box<Stmt<'a>>> {
		&self._else
	}
}

impl std::fmt::Display for If<'_> {
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
