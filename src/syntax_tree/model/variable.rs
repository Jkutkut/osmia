use std::collections::LinkedList;
use crate::lexer::VariableLexer;
use crate::syntax_tree::model::VariableKey;

#[derive(Debug, PartialEq)]
pub struct Variable<'a> {
	raw: &'a str,
	keys: LinkedList<VariableKey<'a>>,
}

impl<'a> Variable<'a> {
	pub fn new(raw: &'a str, lst: LinkedList<VariableKey<'a>>) -> Self {
		Self {
			raw,
			keys: lst
		}
	}

	/// Parses a str into a Variable.
	/// Checks if the variable is valid.
	pub fn from_str(raw: &'a str) -> Option<Self> {
		Some(Self::new(
			raw,
			VariableLexer::lex(raw)?
		))
	}

	pub fn keys(&self) -> &LinkedList<VariableKey<'a>> {
		&self.keys
	}
}

impl std::fmt::Display for Variable<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.raw)
	}
}
