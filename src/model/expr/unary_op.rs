use super::*;

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

impl Display for UnaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			UnaryOp::Plus => write!(f, "+"),
			UnaryOp::Minus => write!(f, "-"),
			UnaryOp::Not => write!(f, "!"),
		}
	}
}
