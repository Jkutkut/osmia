use super::{Literal, Grouping, Unary, Binary};

#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
	Literal(Literal),
	Grouping(Grouping<'a>),
	Unary(Unary<'a>),
	Binary(Binary<'a>)
}
