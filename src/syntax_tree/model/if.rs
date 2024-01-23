use crate::syntax_tree::model::{Block, ConditionalBlock};

#[derive(Debug, PartialEq)]
pub struct If<'a> {
	_if: ConditionalBlock<'a>,
	_elseif: Option<Vec<ConditionalBlock<'a>>>,
	_else: Option<Block<'a>>,
}

impl std::fmt::Display for If<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "if {}", self._if)?;
		if let Some(elseifs) = &self._elseif {
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
