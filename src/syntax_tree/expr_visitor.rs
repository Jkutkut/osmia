use std::collections::HashMap;
use crate::model::{
	Expression, JsonExpression,
	Literal, Unary, Binary, Grouping, Variable,
	Call, MethodCall
};

pub trait ExprVisitor<T> {
	// Json
	fn visit_array(&self, arr: &Vec<JsonExpression>) -> T;
	fn visit_object(&self, obj: &HashMap<String, JsonExpression>) -> T;

	// Callable
	fn visit_call(&self, call: &Call) -> T;
	fn visit_method_call(&self, method_call: &MethodCall) -> T;

	// Expression
	#[allow(unused)]
	fn visit_expression(&self, expression: &Expression) -> T;
	fn visit_literal(&self, literal: &Literal) -> T;
	fn visit_variable(&self, literal: &Variable) -> T;
	fn visit_grouping(&self, grouping: &Grouping) -> T;
	fn visit_unary(&self, unary: &Unary) -> T;
	fn visit_binary(&self, binary: &Binary) -> T;
}
