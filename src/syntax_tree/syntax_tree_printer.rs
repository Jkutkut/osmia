use crate::Token;
use super::visitor::Visitor;
use super::visitable::Visitable;
use super::model::{
	Expression, Literal, Unary, Binary, Grouping, Variable,
	Stmt, Block, Assign, ConditionalBlock
};

pub struct SyntaxTreePrinter;

impl Visitor<String> for SyntaxTreePrinter {
	// Stmt
	fn visit_stmt(&self, stmt: &Stmt) -> String {
		stmt.accept(self)
	}

	fn visit_block(&self, block: &Block) -> String {
		block.stmts()
			.iter()
			.map(|stmt| stmt.accept(self))
			.collect::<Vec<String>>()
			.join("; ")
	}

	fn visit_raw(&self, raw: &str) -> String {
		raw.to_string()
	}

	// TODO print

	fn visit_assign(&self, assign: &Assign) -> String {
		format!("{} = {}", assign.variable(), assign.expression().accept(self))
	}

	fn visit_while(&self, block: &ConditionalBlock) -> String {
		format!("while {}", block.accept(self))
	}

	fn visit_conditional_block(&self, block: &ConditionalBlock) -> String {
		format!(
			"( {} ) {{ {} }}",
			block.condition().accept(self),
			block.body().accept(self)
		)
	}

	fn visit_break(&self) -> String {
		Token::Break.to_string()
	}

	fn visit_continue(&self) -> String {
		Token::Continue.to_string()
	}

	// Expression
	fn visit_expression(&self, expression: &Expression) -> String {
		expression.accept(self)
	}

	fn visit_literal(&self, literal: &Literal) -> String {
		match literal {
			Literal::Float(f) => f.to_string(),
			Literal::Int(i) => i.to_string(),
			Literal::Str(s) => s.to_string(),
			Literal::Bool(b) => b.to_string(),
			Literal::Null => "null".to_string()
		}
	}

	fn visit_variable(&self, variable: &Variable) -> String {
		variable.to_string()
	}

	fn visit_grouping(&self, grouping: &Grouping) -> String {
		format!("({})", &grouping.expression.accept(self))
	}

	fn visit_unary(&self, unary: &Unary) -> String {
		format!("{}{}", &unary.operator, &unary.right.accept(self))
	}

	fn visit_binary(&self, binary: &Binary) -> String {
		format!("{} {} {}", &binary.left.accept(self), &binary.operator, &binary.right.accept(self))
	}
}
