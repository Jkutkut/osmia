use crate::syntax_tree::model::{
	Expression, Literal, Unary, Binary, Grouping, Variable, Stmt
};

pub trait Visitor<T> {
	fn visit_stmt(&self, stmt: &Stmt) -> T;
	fn visit_expression(&self, expression: &Expression) -> T;
	fn visit_literal(&self, literal: &Literal) -> T;
	fn visit_variable(&self, literal: &Variable) -> T;
	fn visit_grouping(&self, grouping: &Grouping) -> T;
	fn visit_unary(&self, unary: &Unary) -> T;
	fn visit_binary(&self, binary: &Binary) -> T;
}
