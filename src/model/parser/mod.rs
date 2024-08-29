mod parser;
mod osmia_parser;

pub use parser::Parser;
pub use osmia_parser::OsmiaParser;

// -----------------------------------
use crate::model::lexer::Token;

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

impl From<Token> for Option<BinaryOp> {
	fn from(token: Token) -> Self {
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

pub enum UnaryOp {
	Plus,
	Minus,
	Not,
}

impl From<Token> for Option<UnaryOp> {
	fn from(token: Token) -> Self {
		Some(match token {
			Token::Plus => UnaryOp::Plus,
			Token::Minus => UnaryOp::Minus,
			Token::Not => UnaryOp::Not,
			_ => return None
		})
	}
}
