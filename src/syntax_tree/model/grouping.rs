use super::Expression;

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
