use super::Expression;
use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Binary<'a> {
	pub left: Box<Expression<'a>>,
	pub operator: Token<'a>,
	pub right: Box<Expression<'a>>
}

impl<'a> Binary<'a> {
	pub fn new(
		left: Expression<'a>,
		operator: Token<'a>,
		right: Expression<'a>
	) -> Result<Binary<'a>, String> {
		if !operator.is_binary_operator() {
			return Err(format!("Invalid binary operator: {}", operator));
		}
		Ok(Binary {
			left: Box::new(left),
			operator: operator,
			right: Box::new(right)
		})
	}
}

impl std::fmt::Display for Binary<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {} {}", self.operator, self.left, self.right)
	}
}
