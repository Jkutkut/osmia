use crate::model::{
	Expression,
	Stmt, Block, Assign, ConditionalBlock, ForEach, If
};

pub trait StmtVisitor<T> {
	fn visit_stmt(&self, stmt: &Stmt) -> T;
	fn visit_block(&self, block: &Block) -> T;
	fn visit_raw(&self, raw: &str) -> T;
	fn visit_print(&self, print: &Expression) -> T;
	fn visit_assign(&self, assign: &Assign) -> T;
	fn visit_if(&self, block: &If) -> T;
	fn visit_while(&self, block: &ConditionalBlock) -> T;
	fn visit_foreach(&self, block: &ForEach) -> T;
	fn visit_conditional_block(&self, block: &ConditionalBlock) -> T;
	fn visit_break(&self) -> T;
	fn visit_continue(&self) -> T;

	// Expression
	fn visit_expression(&self, expression: &Expression) -> T;
}
