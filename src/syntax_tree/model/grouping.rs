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
