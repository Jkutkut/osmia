use crate::model::{
	JsonExpression,
	Stmt, Block, Assign, ConditionalBlock, ForEach, If
};

pub trait StmtVisitor<T> {
	fn visit_stmt(&mut self, stmt: &Stmt) -> T;
	fn visit_block(&mut self, block: &Block) -> T;
	fn visit_raw(&self, raw: &str) -> T;
	fn visit_print(&self, print: &JsonExpression) -> T;
	fn visit_assign(&mut self, assign: &Assign) -> T;
	fn visit_if(&mut self, block: &If) -> T;
	fn visit_while(&mut self, block: &ConditionalBlock) -> T;
	fn visit_foreach(&mut self, block: &ForEach) -> T;
	fn visit_conditional_block(&mut self, block: &ConditionalBlock) -> T;
	fn visit_break(&self) -> T;
	fn visit_continue(&self) -> T;

	// Expression
	fn visit_expression(&self, expression: &JsonExpression) -> T;
}
