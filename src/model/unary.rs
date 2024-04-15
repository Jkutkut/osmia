use super::Expression;
use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Unary<'a> {
	pub operator: Token<'a>,
	pub right: Box<Expression<'a>>
}

impl<'a> Unary<'a> {
	pub fn new(operator: Token<'a>, right: Expression<'a>) -> Result<Unary<'a>, String> {
		match operator {
			Token::Minus | Token::Not | Token::Plus => Ok(Unary {
				operator: operator,
				right: Box::new(right)
			}),
			_ => return Err(format!("Invalid unary operator: {}", operator))
		}
	}
}

impl std::fmt::Display for Unary<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}{}", self.operator, self.right)
	}
}
