use super::{Literal, Unary, Binary, Grouping, Expression, Variable};
use crate::syntax_tree::visitable::Visitable;
use crate::syntax_tree::visitor::Visitor;

impl<T> Visitable<T> for Expression<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			Expression::Literal(literal) => visitor.visit_literal(literal),
			Expression::Variable(variable) => visitor.visit_variable(variable),
			Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
			Expression::Unary(unary) => visitor.visit_unary(unary),
			Expression::Binary(binary) => visitor.visit_binary(binary),
		}
	}
}

impl<T> Visitable<T> for Literal {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_literal(self)
	}
}

impl<T> Visitable<T> for Variable<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_variable(self)
	}
}

impl<T> Visitable<T> for Grouping<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_grouping(self)
	}
}

impl<T> Visitable<T> for Unary<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_unary(self)
	}
}

impl<T> Visitable<T> for Binary<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_binary(self)
	}
}
