use crate::model::{
	Expression, Literal, Unary, Binary, Grouping, Variable, JsonExpression,
	Callable, Call, MethodCall
};
use crate::syntax_tree::{
	ExprVisitable, ExprVisitor
};

impl<T> ExprVisitable<T> for Literal {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_literal(self)
	}
}

impl<T> ExprVisitable<T> for Variable {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_variable(self)
	}
}

impl<T> ExprVisitable<T> for Grouping {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_grouping(self)
	}
}

impl<T> ExprVisitable<T> for Unary {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_unary(self)
	}
}

impl<T> ExprVisitable<T> for Binary {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_binary(self)
	}
}

impl<T> ExprVisitable<T> for Expression {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		match self {
			Expression::Literal(literal) => literal.accept(visitor),
			Expression::Variable(variable) => variable.accept(visitor),
			Expression::Grouping(grouping) => grouping.accept(visitor),
			Expression::Unary(unary) => unary.accept(visitor),
			Expression::Binary(binary) => binary.accept(visitor),
			Expression::Callable(callable) => callable.accept(visitor)
		}
	}
}

impl<T> ExprVisitable<T> for JsonExpression {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		match self {
			JsonExpression::Array(arr) => visitor.visit_array(arr),
			JsonExpression::Object(obj) => visitor.visit_object(obj),
			JsonExpression::Expression(expr) => visitor.visit_expression(expr),
		}
	}
}

impl<T> ExprVisitable<T> for Callable {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		match self {
			Callable::Call(call) => call.accept(visitor),
			Callable::MethodCall(method) => method.accept(visitor),
		}
	}
}

impl<T> ExprVisitable<T> for Call {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_call(self)
	}
}

impl<T> ExprVisitable<T> for MethodCall {
	fn accept(&self, visitor: &dyn ExprVisitor<T>) -> T {
		visitor.visit_method_call(self)
	}
}
