use crate::syntax_tree::model::{
	Expression, Literal, Unary, Binary, Grouping, Variable
};
use crate::syntax_tree::{
	ExprVisitable, ExprVisitor
};

impl<T> ExprVisitable<T> for Literal {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_literal(self)
	}
}

impl<T> ExprVisitable<T> for Variable<'_> {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_variable(self)
	}
}

impl<T> ExprVisitable<T> for Grouping<'_> {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_grouping(self)
	}
}

impl<T> ExprVisitable<T> for Unary<'_> {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_unary(self)
	}
}

impl<T> ExprVisitable<T> for Binary<'_> {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_binary(self)
	}
}

impl<T> ExprVisitable<T> for Expression<'_> {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		match self {
			Expression::Literal(literal) => literal.accept(visitor),
			Expression::Variable(variable) => variable.accept(visitor),
			Expression::Grouping(grouping) => grouping.accept(visitor),
			Expression::Unary(unary) => unary.accept(visitor),
			Expression::Binary(binary) => binary.accept(visitor),
		}
	}
}
