use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
	op: UnaryOp,
	expr: Box<Expr>,
}

impl Unary {
	pub fn new(op: UnaryOp, expr: Expr) -> Self {
		Self { op, expr: Box::new(expr) }
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
	Plus,
	Minus,
	Not,
}

impl From<&Token> for Option<UnaryOp> {
	fn from(token: &Token) -> Self {
		Some(match token {
			Token::Plus => UnaryOp::Plus,
			Token::Minus => UnaryOp::Minus,
			Token::Not => UnaryOp::Not,
			_ => return None
		})
	}
}
