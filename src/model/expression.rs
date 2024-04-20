use super::{Literal, Grouping, Unary, Binary, Variable};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	Literal(Literal),
	Variable(Variable),
	Grouping(Grouping),
	Unary(Unary),
	Binary(Binary)
}

impl std::fmt::Display for Expression {
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
