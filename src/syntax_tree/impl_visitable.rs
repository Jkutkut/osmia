use crate::model::{
	Expression, Literal, Unary, Binary, Grouping, Variable,
	JsonExpression, ListOrVariable,
	Stmt, Assign, ConditionalBlock, ForEach, If,
	Callable, Call, MethodCall
};
use crate::syntax_tree::{
	Visitable, Visitor
};

// Stmt

impl<T> Visitable<T> for Stmt {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			Stmt::Block(blocks) => visitor.visit_block(blocks),
			Stmt::Raw(raw) => visitor.visit_raw(raw),
			Stmt::Print(print) => visitor.visit_print(print),
			Stmt::Expression(json) => match json {
				JsonExpression::Expression(expr) => expr.accept(visitor),
				JsonExpression::Array(arr) => visitor.visit_array(arr),
				JsonExpression::Object(obj) => visitor.visit_object(obj)
			}
			Stmt::Assign(assign) => assign.accept(visitor),
			Stmt::If(if_stmt) => if_stmt.accept(visitor),
			Stmt::While(while_stmt) => visitor.visit_while(while_stmt),
			Stmt::ForEach(foreach) => foreach.accept(visitor),
			Stmt::Break => visitor.visit_break(),
			Stmt::Continue => visitor.visit_continue(),
		}
	}
}

impl<T> Visitable<T> for Assign {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_assign(self)
	}
}

impl<T> Visitable<T> for If {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_if(self)
	}
}

impl<T> Visitable<T> for ConditionalBlock {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_conditional_block(self)
	}
}

impl<T> Visitable<T> for ForEach {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_foreach(self)
	}
}

// Json

impl<T> Visitable<T> for JsonExpression {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			JsonExpression::Expression(expr) => expr.accept(visitor),
			JsonExpression::Array(arr) => visitor.visit_array(arr),
			JsonExpression::Object(obj) => visitor.visit_object(obj)
		}
	}
}

impl<T> Visitable<T> for ListOrVariable {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			ListOrVariable::List(json) => match json {
				JsonExpression::Array(arr) => visitor.visit_array(arr),
				_ => unreachable!()
			},
			ListOrVariable::Variable(var) => visitor.visit_variable(var)
		}
	}
}

// Expression

impl<T> Visitable<T> for Literal {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_literal(self)
	}
}

impl<T> Visitable<T> for Variable {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_variable(self)
	}
}

impl<T> Visitable<T> for Grouping {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_grouping(self)
	}
}

impl<T> Visitable<T> for Unary {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_unary(self)
	}
}

impl<T> Visitable<T> for Binary {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_binary(self)
	}
}

impl<T> Visitable<T> for Expression {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			Expression::Literal(literal) => literal.accept(visitor),
			Expression::Variable(variable) => variable.accept(visitor),
			Expression::Grouping(grouping) => grouping.accept(visitor),
			Expression::Unary(unary) => unary.accept(visitor),
			Expression::Binary(binary) => binary.accept(visitor),
			Expression::Callable(callable) => callable.accept(visitor),
		}
	}
}

impl<T> Visitable<T> for Callable {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		match self {
			Callable::Call(call) => call.accept(visitor),
			Callable::MethodCall(method) => method.accept(visitor),
		}
	}
}

impl<T> Visitable<T> for Call {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_call(self)
	}
}

impl<T> Visitable<T> for MethodCall {
	fn accept(&self, visitor: &dyn Visitor<T>) -> T {
		visitor.visit_method_call(self)
	}
}
