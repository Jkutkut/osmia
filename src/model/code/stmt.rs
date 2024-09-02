use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
	Block(Block),
	Raw(String),
	Expr(Expr),
	Print(Print),
	Comment(String),
	// Assign(Assign),
	// If(If),
	// While(While),
	// For(For),
	Break,
	Continue,
	Return(Return),
	// Function(Function),
}

// #[derive(Debug, PartialEq, Clone)]
// struct Assign {}

// #[derive(Debug, PartialEq, Clone)]
// struct If {}

// #[derive(Debug, PartialEq, Clone)]
// struct While {}

// #[derive(Debug, PartialEq, Clone)]
// struct For {}

// #[derive(Debug, PartialEq, Clone)]
// struct Return {}

// #[derive(Debug, PartialEq, Clone)]
// struct Function {}

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
