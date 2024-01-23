use super::Expression;

#[derive(Debug, PartialEq)]
pub struct Grouping<'a> {
	pub expression: Box<Expression<'a>>
}

impl<'a> Grouping<'a> {
	pub fn new(expression: Expression<'a>) -> Grouping<'a> {
		Grouping {
			expression: Box::new(expression)
		}
	}
}

impl std::fmt::Display for Grouping<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({})", self.expression)
	}
}
