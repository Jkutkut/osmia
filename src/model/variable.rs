use std::collections::LinkedList;
use crate::lexer::VariableLexer;
use crate::model::VariableKey;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
	raw: String,
	keys: LinkedList<VariableKey>,
}

impl Variable {
	pub fn new(raw: &str, lst: LinkedList<VariableKey>) -> Self {
		Self {
			raw: raw.to_string(),
			keys: lst
		}
	}

	/// Parses a str into a Variable.
	/// Checks if the variable is valid.
	pub fn from_str(raw: &str) -> Option<Self> {
		Some(Self::new(
			raw,
			VariableLexer::lex(raw)?
		))
	}

	pub fn keys(&self) -> &LinkedList<VariableKey> {
		&self.keys
	}
}

impl std::fmt::Display for Variable {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.raw)
	}
}
