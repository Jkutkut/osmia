use super::Expression;
use crate::token::Token;

pub struct Unary<'a> {
	pub operator: Token<'a>,
	pub right: Box<Expression<'a>>
}

impl<'a> Unary<'a> {
	pub fn new(operator: Token<'a>, right: Expression<'a>) -> Result<Unary<'a>, String> {
		match operator {
			Token::Minus | Token::Not => Ok(Unary {
				operator: operator,
				right: Box::new(right)
			}),
			_ => return Err(format!("Invalid unary operator: {}", operator))
		}
	}
}
