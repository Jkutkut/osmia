use crate::Token;
use crate::syntax_tree::{
	Visitor, Visitable
};
use crate::model::{
	Expression, Literal, Unary, Binary, Grouping, Variable,
	Stmt, Block, Assign, ConditionalBlock, ForEach, If
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

	fn visit_print(&self, expr: &Expression) -> String {
		format!("print {}", expr.accept(self))
	}

	fn visit_assign(&self, assign: &Assign) -> String {
		format!("{} = {}", assign.variable(), assign.expression().accept(self))
	}

	fn visit_if(&self, block: &If) -> String {
		let mut result = format!("if {}", block.if_block().accept(self));
		if let Some(else_ifs) = block.elseifs() {
			for else_if in else_ifs {
				result = format!("{} else if {}", result, else_if.accept(self));
			}
		}
		if let Some(else_block) = block.else_block() {
			result = format!("{} else {{ {} }}", result, else_block.accept(self));
		}
		result
	}

	fn visit_while(&self, block: &ConditionalBlock) -> String {
		format!("while {}", block.accept(self))
	}

	fn visit_foreach(&self, block: &ForEach) -> String {
		format!(
			"for ( {} in {} ) {{ {} }}",
			block.variable(),
			block.list(),
			block.body().accept(self)
		)
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
