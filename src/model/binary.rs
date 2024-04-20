use super::Expression;
use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Binary {
	pub left: Box<Expression>,
	pub operator: Token,
	pub right: Box<Expression>
}

impl Binary {
	pub fn new(
		left: Expression,
		operator: Token,
		right: Expression
	) -> Result<Binary, String> {
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

impl std::fmt::Display for Binary {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} {} {}", self.operator, self.left, self.right)
	}
}
