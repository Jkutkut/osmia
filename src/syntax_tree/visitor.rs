use std::collections::HashMap;
use crate::model::{
	Literal, Unary, Binary, Grouping, Variable, JsonExpression,
	Stmt, Block, Assign, ConditionalBlock, ForEach, If,
	Call, MethodCall
};

pub trait Visitor<T> {
	// Stmt
	#[allow(unused)]
	fn visit_stmt(&self, stmt: &Stmt) -> T;
	fn visit_block(&self, block: &Block) -> T;
	fn visit_raw(&self, raw: &str) -> T;
	fn visit_print(&self, print: &JsonExpression) -> T;
	fn visit_assign(&self, assign: &Assign) -> T;
	fn visit_if(&self, block: &If) -> T;
	fn visit_while(&self, block: &ConditionalBlock) -> T;
	fn visit_foreach(&self, block: &ForEach) -> T;
	fn visit_conditional_block(&self, block: &ConditionalBlock) -> T;
	fn visit_break(&self) -> T;
	fn visit_continue(&self) -> T;

	// Json
	fn visit_array(&self, arr: &Vec<JsonExpression>) -> T;
	fn visit_object(&self, obj: &HashMap<String, JsonExpression>) -> T;

	// Expression
	fn visit_literal(&self, literal: &Literal) -> T;
	fn visit_variable(&self, literal: &Variable) -> T;
	fn visit_grouping(&self, grouping: &Grouping) -> T;
	fn visit_unary(&self, unary: &Unary) -> T;
	fn visit_binary(&self, binary: &Binary) -> T;

	// Callable
	fn visit_call(&self, call: &Call) -> T;
	fn visit_method_call(&self, method_call: &MethodCall) -> T;
}
