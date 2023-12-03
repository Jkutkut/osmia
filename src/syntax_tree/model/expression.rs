use super::{Literal, Grouping, Unary, Binary};

pub enum Expression<'a> {
	Literal(Literal),
	Grouping(Grouping<'a>),
	Unary(Unary<'a>),
	Binary(Binary<'a>)
}
