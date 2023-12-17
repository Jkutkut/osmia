use super::visitor::Visitor;
use super::visitable::Visitable;
use super::model::{Expression, Literal, Unary, Binary, Grouping, Variable};

pub struct SyntaxTreePrinter;

impl Visitor<String> for SyntaxTreePrinter {
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
