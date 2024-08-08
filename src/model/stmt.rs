use crate::model::{
	Block, Assign, If, ConditionalBlock, ForEach, JsonExpression
};

#[derive(Debug, PartialEq)]
pub enum Stmt {
	Block(Block),
	Raw(String),
	Print(JsonExpression),
	Expression(JsonExpression),
	Assign(Assign),
	If(If),
	While(ConditionalBlock),
	ForEach(ForEach),
	Break,
	Continue,
}

impl std::fmt::Display for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Stmt::Break => write!(f, "break"),
			Stmt::Continue => write!(f, "continue"),
			Stmt::Expression(json) => write!(f, "${{{}}};", json),
			Stmt::Print(json) => write!(f, "print({});", json),
			_ => write!(f, "{}", self),
		}
	}
}
