use super::{
	Literal, Unary, Binary, Grouping, Variable, Stmt,
	Expression
};
use crate::syntax_tree::visitable::Visitable;
use crate::syntax_tree::visitor::Visitor;

impl<T> Visitable<T> for Stmt<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			// Stmt::Block(block) => block.accept(visitor),
			// Stmt::Raw(raw) => raw.accept(visitor),
			// Stmt::Print(print) => print.accept(visitor),
			Stmt::Expression(expression) => expression.accept(visitor),
			// Stmt::Assign(assign) => assign.accept(visitor),
			// Stmt::If(if_stmt) => if_stmt.accept(visitor),
			// Stmt::While(while_stmt) => while_stmt.accept(visitor),
			// Stmt::ForEach(for_each) => for_each.accept(visitor),
			// Stmt::Break => visitor.visit_break(),
			// Stmt::Continue => visitor.visit_continue(),
			_ => todo!()
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

impl<T> Visitable<T> for Expression<'_> {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			Expression::Literal(literal) => literal.accept(visitor),
			Expression::Variable(variable) => variable.accept(visitor),
			Expression::Grouping(grouping) => grouping.accept(visitor),
			Expression::Unary(unary) => unary.accept(visitor),
			Expression::Binary(binary) => binary.accept(visitor),
		}
	}
}
