use super::Expression;
use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Unary {
	pub operator: Token,
	pub right: Box<Expression>
}

impl Unary {
	pub fn new(operator: Token, right: Expression) -> Result<Unary, String> {
		match operator {
			Token::Minus | Token::Not | Token::Plus => Ok(Unary {
				operator: operator,
				right: Box::new(right)
			}),
			_ => return Err(format!("Invalid unary operator: {}", operator))
		}
	}
}

impl std::fmt::Display for Unary {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}{}", self.operator, self.right)
	}
}
