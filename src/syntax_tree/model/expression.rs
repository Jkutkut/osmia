use super::{Literal, Grouping, Unary, Binary, Variable};

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
	Literal(Literal),
	Variable(Variable<'a>),
	Grouping(Grouping<'a>),
	Unary(Unary<'a>),
	Binary(Binary<'a>)
}
