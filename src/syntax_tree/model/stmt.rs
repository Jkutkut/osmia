use crate::syntax_tree::model::{
	Expression, Block, Assign, If, ConditionalBlock, ForEach
};

#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
	Block(Block<'a>),
	Raw(&'a str),
	Print(Expression<'a>),
	Expression(Expression<'a>),
	Assign(Assign<'a>),
	If(If<'a>),
	While(ConditionalBlock<'a>),
	ForEach(ForEach<'a>),
	Break,
	Continue,
}

impl std::fmt::Display for Stmt<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Stmt::Break => write!(f, "break"),
			Stmt::Continue => write!(f, "continue"),
			Stmt::Expression(expr) => write!(f, "${{{}}};", expr),
			Stmt::Print(expr) => write!(f, "print({});", expr),
			_ => write!(f, "{}", self),
		}
	}
}
