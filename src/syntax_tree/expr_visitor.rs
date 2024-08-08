use std::collections::HashMap;
use crate::model::{
	Expression, JsonExpression,
	Literal, Unary, Binary, Grouping, Variable,
};

pub trait ExprVisitor<T> {
	fn visit_array(&self, arr: &Vec<JsonExpression>) -> T;
	fn visit_object(&self, obj: &HashMap<String, JsonExpression>) -> T;

	#[allow(unused)]
	fn visit_expression(&self, expression: &Expression) -> T;
	fn visit_literal(&self, literal: &Literal) -> T;
	fn visit_variable(&self, literal: &Variable) -> T;
	fn visit_grouping(&self, grouping: &Grouping) -> T;
	fn visit_unary(&self, unary: &Unary) -> T;
	fn visit_binary(&self, binary: &Binary) -> T;
}
