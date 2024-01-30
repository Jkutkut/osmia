use crate::syntax_tree::model::{
	Expression, Literal, Unary, Binary, Grouping, Variable,
	Stmt, Block, Assign, ConditionalBlock, ForEach
};

pub trait Visitor<T> {
	// Stmt
	fn visit_stmt(&self, stmt: &Stmt) -> T;
	fn visit_block(&self, block: &Block) -> T;
	fn visit_raw(&self, raw: &str) -> T;
	fn visit_print(&self, print: &Expression) -> T;
	fn visit_assign(&self, assign: &Assign) -> T;
	// TODO
	fn visit_while(&self, block: &ConditionalBlock) -> T;
	fn visit_foreach(&self, block: &ForEach) -> T;
	fn visit_conditional_block(&self, block: &ConditionalBlock) -> T;
	fn visit_break(&self) -> T;
	fn visit_continue(&self) -> T;

	// Expression
	fn visit_expression(&self, expression: &Expression) -> T;
	fn visit_literal(&self, literal: &Literal) -> T;
	fn visit_variable(&self, literal: &Variable) -> T;
	fn visit_grouping(&self, grouping: &Grouping) -> T;
	fn visit_unary(&self, unary: &Unary) -> T;
	fn visit_binary(&self, binary: &Binary) -> T;
}
