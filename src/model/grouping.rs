use super::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
	pub expression: Box<Expression>
}

impl Grouping {
	pub fn new(expression: Expression) -> Grouping {
		Grouping {
			expression: Box::new(expression)
		}
	}
}

impl std::fmt::Display for Grouping {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({})", self.expression)
	}
}
