use super::{Literal, Grouping, Unary, Binary, Variable};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
	Literal(Literal),
	Variable(Variable<'a>),
	Grouping(Grouping<'a>),
	Unary(Unary<'a>),
	Binary(Binary<'a>)
}

impl std::fmt::Display for Expression<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Literal(l) => write!(f, "{}", l),
			Expression::Variable(v) => write!(f, "{}", v),
			Expression::Grouping(g) => write!(f, "{}", g),
			Expression::Unary(u) => write!(f, "{}", u),
			Expression::Binary(b) => write!(f, "{}", b)
		}
	}
}
