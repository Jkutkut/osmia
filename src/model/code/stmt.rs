use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
	Block(Block),
	Raw(String),
	NewLine,
	Expr(Expr),
	Print(Print),
	Comment(String),
	Assign(Assign),
	If(If),
	While(While),
	For(For),
	Break,
	Continue,
	Return(Return),
	Function(Function),
}

impl Stmt {
	pub fn new_raw(s: &str) -> Self {
		Self::Raw(s.to_string())
	}

	pub fn new_print(expr: Expr) -> Self {
		Self::Print(Print::new(expr))
	}

	pub fn new_comment(s: &str) -> Self {
		Self::Comment(s.to_string())
	}

	pub fn new_assign(var: Variable, expr: Expr) -> Self {
		Self::Assign(Assign::new(var, expr))
	}

	pub fn new_return(expr: Option<Expr>) -> Self {
		Self::Return(Return::new(expr))
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

impl From<While> for Stmt {
	fn from(w: While) -> Self {
		Self::While(w)
	}
}
