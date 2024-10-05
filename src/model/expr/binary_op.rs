use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
	Equal,
	NotEqual,
	BitAnd,
	BitOr,
	BitXor,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	BitShiftLeft,
	BitShiftRight,
	Plus,
	Minus,
	Mult,
	Div,
	Mod,
	And,
	Or
}

impl From<&Token> for Option<BinaryOp> {
	fn from(token: &Token) -> Self {
		Some(match token {
			Token::Equal => BinaryOp::Equal,
			Token::NotEqual => BinaryOp::NotEqual,
			Token::BitAnd => BinaryOp::BitAnd,
			Token::BitOr => BinaryOp::BitOr,
			Token::BitXor => BinaryOp::BitXor,
			Token::Greater => BinaryOp::Greater,
			Token::GreaterEqual => BinaryOp::GreaterEqual,
			Token::Less => BinaryOp::Less,
			Token::LessEqual => BinaryOp::LessEqual,
			Token::BitShiftLeft => BinaryOp::BitShiftLeft,
			Token::BitShiftRight => BinaryOp::BitShiftRight,
			Token::Plus => BinaryOp::Plus,
			Token::Minus => BinaryOp::Minus,
			Token::Mult => BinaryOp::Mult,
			Token::Div => BinaryOp::Div,
			Token::Mod => BinaryOp::Mod,
			Token::And => BinaryOp::And,
			Token::Or => BinaryOp::Or,
			_ => return None
		})
	}
}

impl Display for BinaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			BinaryOp::Equal => write!(f, "=="),
			BinaryOp::NotEqual => write!(f, "!="),
			BinaryOp::BitAnd => write!(f, "&"),
			BinaryOp::BitOr => write!(f, "|"),
			BinaryOp::BitXor => write!(f, "^"),
			BinaryOp::Greater => write!(f, ">"),
			BinaryOp::GreaterEqual => write!(f, ">="),
			BinaryOp::Less => write!(f, "<"),
			BinaryOp::LessEqual => write!(f, "<="),
			BinaryOp::BitShiftLeft => write!(f, "<<"),
			BinaryOp::BitShiftRight => write!(f, ">>"),
			BinaryOp::Plus => write!(f, "+"),
			BinaryOp::Minus => write!(f, "-"),
			BinaryOp::Mult => write!(f, "*"),
			BinaryOp::Div => write!(f, "/"),
			BinaryOp::Mod => write!(f, "%"),
			BinaryOp::And => write!(f, "&&"),
			BinaryOp::Or => write!(f, "||"),
		}
	}
}
